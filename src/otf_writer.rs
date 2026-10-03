pub mod stat;

use crate::support::buffer::Buffer;
use crate::support::options::Options;

use crate::font::caryll_font::{Font, FontSubtype};
use crate::font::table_registry::BUILD_ORDER;
use crate::font::caryll_sfnt_builder::{
    SfntBuilder, sfnt_builder_push_table, sfnt_builder_serialize,
};
use crate::otf_writer::stat::{stat_font, unstat_font};

/// Serializes a consolidated font into sfnt (OTF/TTF) bytes.
///
/// Was a `FontSerializer` impl on a zero-sized `OtfSerializer` marker
/// struct plus a casting wrapper; see `otf_reader::read_otf` for why that
/// trait is gone. With the erased return type went the reason to hand back
/// a `Buffer::into_raw` pointer -- only `ffi/dll.rs`'s genuine `extern "C"`
/// boundary needs one, and it makes that conversion itself now.
pub fn serialize_to_otf(font: &mut Font, options: &Options) -> Buffer {
    stat_font(&mut *font, options);
    let mut builder = SfntBuilder::new(
        (if font.subtype == FontSubtype::Cff {
            crate::tag::SFNT_VERSION_OTTO as i32
        } else {
            crate::tag::SFNT_VERSION_TRUE_TYPE as i32
        }) as u32,
        options,
    );
    for table in BUILD_ORDER {
        table.build(font, &mut builder, options);
    }
    if options.dummy_dsig {
        let mut dsig = Buffer::new();
        dsig.write_u32be(0x1_u32);
        dsig.write_u16be(0_u16);
        dsig.write_u16be(0_u16);
        sfnt_builder_push_table(&mut builder, crate::tag::TAG_DSIG, Some(dsig));
    }
    let otf: Buffer = sfnt_builder_serialize(&builder);
    unstat_font(&mut *font);
    return otf;
}
