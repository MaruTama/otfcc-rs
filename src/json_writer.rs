use crate::support::options::Options;

use crate::font::model::Font;
use crate::font::table_registry::DUMP_ORDER;
use crate::support::built_json::BuiltValue;

/// Dumps a consolidated font into the JSON value tree otfccdump prints.
///
/// Was a `FontSerializer` impl on a zero-sized `JsonSerializer` marker
/// struct plus a casting wrapper; see `otf_reader::read_otf` for why that
/// trait is gone. Returning the `BuiltValue` itself rather than a
/// `BuiltValue::into_raw` pointer drops the last reason that bridge
/// existed on this path.
pub fn serialize_to_json(font: &mut Font, options: &Options) -> BuiltValue {
    let mut root = BuiltValue::new_object(48);
    for table in DUMP_ORDER {
        table.dump(font, &mut root, options);
    }
    return root;
}
