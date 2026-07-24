extern "C" {
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __uint16_t = u16;
pub type uint16_t = __uint16_t;
pub type sds = *mut ::core::ffi::c_char;
pub type glyphid_t = uint16_t;
pub type handle_state = ::core::ffi::c_uint;
pub const HANDLE_STATE_CONSOLIDATED: handle_state = 3;
pub const HANDLE_STATE_NAME: handle_state = 2;
pub const HANDLE_STATE_INDEX: handle_state = 1;
pub const HANDLE_STATE_EMPTY: handle_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_Handle {
    pub state: handle_state,
    pub index: glyphid_t,
    pub name: sds,
}
pub type otfcc_GlyphHandle = otfcc_Handle;
pub type otfcc_LookupHandle = otfcc_Handle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otfcc_HandlePackage {
    pub init: Option<unsafe extern "C" fn(*mut otfcc_Handle) -> ()>,
    pub copy: Option<unsafe extern "C" fn(*mut otfcc_Handle, *const otfcc_Handle) -> ()>,
    pub move_0: Option<unsafe extern "C" fn(*mut otfcc_Handle, *mut otfcc_Handle) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut otfcc_Handle) -> ()>,
    pub replace: Option<unsafe extern "C" fn(*mut otfcc_Handle, otfcc_Handle) -> ()>,
    pub copyReplace: Option<unsafe extern "C" fn(*mut otfcc_Handle, otfcc_Handle) -> ()>,
    pub empty: Option<unsafe extern "C" fn() -> otfcc_Handle>,
    pub dup: Option<unsafe extern "C" fn(otfcc_Handle) -> otfcc_Handle>,
    pub fromIndex: Option<unsafe extern "C" fn(glyphid_t) -> otfcc_Handle>,
    pub fromName: Option<unsafe extern "C" fn(sds) -> otfcc_Handle>,
    pub fromConsolidated: Option<unsafe extern "C" fn(glyphid_t, sds) -> otfcc_Handle>,
    pub consolidateTo: Option<unsafe extern "C" fn(*mut otfcc_Handle, glyphid_t, sds) -> ()>,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn initHandle(mut h: *mut otfcc_Handle) {
    (*h).state = HANDLE_STATE_EMPTY;
    (*h).index = 0 as glyphid_t;
    (*h).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[inline]
unsafe extern "C" fn disposeHandle(mut h: *mut otfcc_Handle) {
    if !(*h).name.is_null() {
        sdsfree((*h).name);
        (*h).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    (*h).index = 0 as glyphid_t;
    (*h).state = HANDLE_STATE_EMPTY;
}
unsafe extern "C" fn copyHandle(mut dst: *mut otfcc_Handle, mut src: *const otfcc_Handle) {
    (*dst).state = (*src).state;
    (*dst).index = (*src).index;
    if !(*src).name.is_null() {
        (*dst).name = sdsdup((*src).name);
    } else {
        (*dst).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_Handle_empty() -> otfcc_Handle {
    let mut x: otfcc_Handle = otfcc_Handle {
        state: HANDLE_STATE_EMPTY,
        index: 0,
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    otfcc_Handle_init(&raw mut x);
    return x;
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_Handle_copy(mut dst: *mut otfcc_Handle, mut src: *const otfcc_Handle) {
    copyHandle(dst, src);
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_Handle_copyReplace(mut dst: *mut otfcc_Handle, src: otfcc_Handle) {
    otfcc_Handle_dispose(dst);
    otfcc_Handle_copy(dst, &raw const src);
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_Handle_dup(src: otfcc_Handle) -> otfcc_Handle {
    let mut dst: otfcc_Handle = otfcc_Handle {
        state: HANDLE_STATE_EMPTY,
        index: 0,
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    otfcc_Handle_copy(&raw mut dst, &raw const src);
    return dst;
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_Handle_init(mut x: *mut otfcc_Handle) {
    initHandle(x);
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_Handle_dispose(mut x: *mut otfcc_Handle) {
    disposeHandle(x as *mut otfcc_Handle);
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_Handle_replace(mut dst: *mut otfcc_Handle, src: otfcc_Handle) {
    otfcc_Handle_dispose(dst);
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw const src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otfcc_Handle>() as size_t,
    );
}
#[inline]
pub(crate) unsafe extern "C" fn otfcc_Handle_move(mut dst: *mut otfcc_Handle, mut src: *mut otfcc_Handle) {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<otfcc_Handle>() as size_t,
    );
    otfcc_Handle_init(src);
}
pub(crate) unsafe extern "C" fn handle_fromIndex(mut id: glyphid_t) -> otfcc_Handle {
    let mut h: otfcc_Handle = otfcc_Handle {
        state: HANDLE_STATE_INDEX,
        index: id,
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    return h;
}
pub(crate) unsafe extern "C" fn handle_fromName(mut s: sds) -> otfcc_Handle {
    let mut h: otfcc_Handle = otfcc_Handle {
        state: HANDLE_STATE_EMPTY,
        index: 0 as glyphid_t,
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    if !s.is_null() {
        h.state = HANDLE_STATE_NAME;
        h.name = s;
    }
    return h;
}
pub(crate) unsafe extern "C" fn handle_fromConsolidated(mut id: glyphid_t, mut s: sds) -> otfcc_Handle {
    let mut h: otfcc_Handle = otfcc_Handle {
        state: HANDLE_STATE_CONSOLIDATED,
        index: id,
        name: sdsdup(s),
    };
    return h;
}
pub(crate) unsafe extern "C" fn handle_consolidateTo(
    mut h: *mut otfcc_Handle,
    mut id: glyphid_t,
    mut name: sds,
) {
    otfcc_Handle_dispose(h as *mut otfcc_Handle);
    (*h).state = HANDLE_STATE_CONSOLIDATED;
    (*h).index = id;
    (*h).name = sdsdup(name);
}
#[no_mangle]
pub static mut otfcc_iHandle: otfcc_HandlePackage = {
    otfcc_HandlePackage {
        init: Some(otfcc_Handle_init as unsafe extern "C" fn(*mut otfcc_Handle) -> ()),
        copy: Some(
            otfcc_Handle_copy as unsafe extern "C" fn(*mut otfcc_Handle, *const otfcc_Handle) -> (),
        ),
        move_0: Some(
            otfcc_Handle_move as unsafe extern "C" fn(*mut otfcc_Handle, *mut otfcc_Handle) -> (),
        ),
        dispose: Some(otfcc_Handle_dispose as unsafe extern "C" fn(*mut otfcc_Handle) -> ()),
        replace: Some(
            otfcc_Handle_replace as unsafe extern "C" fn(*mut otfcc_Handle, otfcc_Handle) -> (),
        ),
        copyReplace: Some(
            otfcc_Handle_copyReplace as unsafe extern "C" fn(*mut otfcc_Handle, otfcc_Handle) -> (),
        ),
        empty: Some(otfcc_Handle_empty),
        dup: Some(otfcc_Handle_dup as unsafe extern "C" fn(otfcc_Handle) -> otfcc_Handle),
        fromIndex: Some(handle_fromIndex as unsafe extern "C" fn(glyphid_t) -> otfcc_Handle),
        fromName: Some(handle_fromName as unsafe extern "C" fn(sds) -> otfcc_Handle),
        fromConsolidated: Some(
            handle_fromConsolidated as unsafe extern "C" fn(glyphid_t, sds) -> otfcc_Handle,
        ),
        consolidateTo: Some(
            handle_consolidateTo as unsafe extern "C" fn(*mut otfcc_Handle, glyphid_t, sds) -> (),
        ),
    }
};
