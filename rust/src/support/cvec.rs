#![allow(unsafe_op_in_unsafe_fn)] // Stage 6 removes this; see rust/README.md
// Generic implementation of otfcc's C-style growable vector
// (`length`/`capacity`/`items: *mut T`), factored out of the per-container
// boilerplate (`subtable_gsub_multi_grow_to`, `_resizeTo`, `_grow`, `_init`,
// `_push`, `_pop`, `_move`, and their ~65 siblings across every other
// container type in the crate). c2rust generated one full copy of this
// arithmetic per container element type, since the original C used an
// X-macro to stamp out a distinct named struct + function family per type
// rather than a single generic implementation.
//
// `CVecRaw<T>` is layout-compatible with every one of those per-type
// structs (`#[repr(C)] { length: usize, capacity: usize, items: *mut T }`,
// in that field order) -- callers cast their own container's pointer to
// `*mut CVecRaw<ElementType>` to use these functions, without needing to
// change the original struct's definition (which stays independently
// duplicated across dozens of files, exactly as c2rust emitted it; only the
// *implementation* of the handful of functions that operate on it changes).
//
// Only the callback-free, purely structural operations are generic here:
// growth/capacity math, push/pop by value, and move. Operations that need
// per-element behavior (custom dispose via a container's own `typeinfo`
// struct, custom init, sort/filter_env taking element-specific function
// pointers) stay written out per container, since genericizing callback
// dispatch is a separate, larger design question than this arithmetic.

use libc::{calloc, realloc};

const INITIAL_SIZE: usize = 2;

#[repr(C)]
pub(crate) struct CVecRaw<T> {
    pub length: usize,
    pub capacity: usize,
    pub items: *mut T,
}
// Copy/Clone regardless of T: every field is either a plain integer or a
// raw pointer, both Copy unconditionally -- matches the semantics of the
// original `#[derive(Copy, Clone)]` per-type structs.
impl<T> Clone for CVecRaw<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for CVecRaw<T> {}

#[inline]
pub(crate) unsafe fn cvec_init<T>(arr: *mut CVecRaw<T>) {
    (*arr).length = 0;
    (*arr).capacity = 0;
    (*arr).items = ::core::ptr::null_mut();
}

#[inline]
pub(crate) unsafe fn cvec_grow_to<T>(arr: *mut CVecRaw<T>, target: usize) {
    if target <= (*arr).capacity {
        return;
    }
    if (*arr).capacity < INITIAL_SIZE {
        (*arr).capacity = INITIAL_SIZE;
    }
    while (*arr).capacity < target {
        (*arr).capacity = (*arr).capacity.wrapping_add((*arr).capacity.wrapping_div(2));
    }
    cvec_realloc_items(arr);
}

#[inline]
pub(crate) unsafe fn cvec_grow_to_n<T>(arr: *mut CVecRaw<T>, target: usize) {
    if target <= (*arr).capacity {
        return;
    }
    if (*arr).capacity < INITIAL_SIZE {
        (*arr).capacity = INITIAL_SIZE;
    }
    if (*arr).capacity < target {
        (*arr).capacity = target.wrapping_add(1);
    }
    cvec_realloc_items(arr);
}

#[inline]
pub(crate) unsafe fn cvec_resize_to<T>(arr: *mut CVecRaw<T>, target: usize) {
    (*arr).capacity = target;
    cvec_realloc_items(arr);
}

#[inline]
unsafe fn cvec_realloc_items<T>(arr: *mut CVecRaw<T>) {
    let bytes = (*arr).capacity.wrapping_mul(::core::mem::size_of::<T>());
    (*arr).items = if !(*arr).items.is_null() {
        realloc((*arr).items as *mut ::core::ffi::c_void, bytes) as *mut T
    } else {
        calloc((*arr).capacity, ::core::mem::size_of::<T>()) as *mut T
    };
}

#[inline]
pub(crate) unsafe fn cvec_grow<T>(arr: *mut CVecRaw<T>) {
    cvec_grow_to(arr, (*arr).length.wrapping_add(1));
}

// No `T: Copy` bound: `ptr::write`/`ptr::read` move the value in and out of
// the backing allocation without going through an implicit Copy, which is
// what lets a container hold a non-Copy element (e.g. `Point`, once its `VQ`
// fields own a real `Vec`). Byte-identical to the old assignment/dereference
// form for the Copy element types that still use this.
#[inline]
pub(crate) unsafe fn cvec_push<T>(arr: *mut CVecRaw<T>, elem: T) {
    cvec_grow(arr);
    let fresh = (*arr).length;
    (*arr).length = (*arr).length.wrapping_add(1);
    (*arr).items.offset(fresh as isize).write(elem);
}

#[inline]
pub(crate) unsafe fn cvec_pop<T>(arr: *mut CVecRaw<T>) -> T {
    let t = (*arr).items.offset((*arr).length.wrapping_sub(1) as isize).read();
    (*arr).length = (*arr).length.wrapping_sub(1);
    t
}

#[inline]
pub(crate) unsafe fn cvec_move<T>(dst: *mut CVecRaw<T>, src: *mut CVecRaw<T>) {
    *dst = *src;
    cvec_init(src);
}

// These tests pin down the *observable capacity arithmetic*, not just the
// happy path. That matters because this one implementation backs all ~37
// container types in the crate, and replacing it with `Vec<T>` (the largest
// single step of the safe-Rust conversion) must not change how many bytes get
// allocated at each step: a container's `capacity` is read directly by
// surrounding code, and any growth-policy drift would change allocation
// patterns that the byte-for-byte comparison against the C build cannot see.
#[cfg(test)]
mod tests {
    use super::*;

    unsafe extern "C" {
        fn free(ptr: *mut ::core::ffi::c_void);
    }

    fn empty<T>() -> CVecRaw<T> {
        CVecRaw {
            length: 0,
            capacity: 0,
            items: ::core::ptr::null_mut(),
        }
    }

    /// Frees the backing allocation so the tests don't leak.
    unsafe fn release<T>(v: &mut CVecRaw<T>) {
        if !v.items.is_null() {
            free(v.items as *mut ::core::ffi::c_void);
        }
        cvec_init(v);
    }

    #[test]
    fn init_clears_all_three_fields() {
        let mut v: CVecRaw<u32> = CVecRaw {
            length: 7,
            capacity: 9,
            items: 0x1 as *mut u32,
        };
        unsafe { cvec_init(&mut v) };
        assert_eq!(v.length, 0);
        assert_eq!(v.capacity, 0);
        assert!(v.items.is_null());
    }

    #[test]
    fn push_grows_capacity_by_one_and_a_half() {
        // 0 -> 2 (INITIAL_SIZE), then cap += cap / 2 until it covers the
        // target. Recorded per push so a policy change can't slip through.
        let expected = [2usize, 2, 3, 4, 6, 6, 9, 9, 9, 13];
        let mut v: CVecRaw<u32> = empty();
        unsafe {
            for (i, want) in expected.iter().enumerate() {
                cvec_push(&mut v, i as u32);
                assert_eq!(v.capacity, *want, "capacity after {} push(es)", i + 1);
                assert_eq!(v.length, i + 1);
            }
            release(&mut v);
        }
    }

    #[test]
    fn grow_to_is_a_noop_when_already_large_enough() {
        let mut v: CVecRaw<u32> = empty();
        unsafe {
            cvec_grow_to(&mut v, 4);
            assert_eq!(v.capacity, 4);
            let items = v.items;
            cvec_grow_to(&mut v, 4);
            cvec_grow_to(&mut v, 1);
            assert_eq!(v.capacity, 4);
            assert_eq!(v.items, items, "must not reallocate");
            release(&mut v);
        }
    }

    #[test]
    fn grow_to_n_overshoots_the_target_by_one() {
        // Distinct from cvec_grow_to: the "_n" variant jumps straight to
        // target + 1 instead of stepping by 1.5x.
        let mut v: CVecRaw<u32> = empty();
        unsafe {
            cvec_grow_to_n(&mut v, 5);
            assert_eq!(v.capacity, 6);
            release(&mut v);
        }
    }

    #[test]
    fn resize_to_sets_capacity_exactly() {
        let mut v: CVecRaw<u32> = empty();
        unsafe {
            cvec_resize_to(&mut v, 5);
            assert_eq!(v.capacity, 5, "no rounding, unlike grow_to");
            release(&mut v);
        }
    }

    #[test]
    fn push_then_pop_returns_values_in_reverse() {
        let mut v: CVecRaw<u16> = empty();
        unsafe {
            for x in [10u16, 20, 30] {
                cvec_push(&mut v, x);
            }
            assert_eq!(cvec_pop(&mut v), 30);
            assert_eq!(cvec_pop(&mut v), 20);
            assert_eq!(v.length, 1);
            // Popping only rewinds `length`; the capacity stays put.
            assert_eq!(v.capacity, 3);
            release(&mut v);
        }
    }

    #[test]
    fn move_transfers_the_allocation_and_resets_the_source() {
        let mut src: CVecRaw<u32> = empty();
        let mut dst: CVecRaw<u32> = empty();
        unsafe {
            cvec_push(&mut src, 42);
            let items = src.items;
            cvec_move(&mut dst, &mut src);
            assert_eq!(dst.items, items);
            assert_eq!(dst.length, 1);
            assert_eq!(*dst.items, 42);
            assert!(src.items.is_null(), "source must not alias the allocation");
            assert_eq!(src.length, 0);
            assert_eq!(src.capacity, 0);
            release(&mut dst);
        }
    }
}

pub const __CARYLL_VECTOR_INITIAL_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
