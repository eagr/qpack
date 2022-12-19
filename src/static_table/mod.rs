mod table;
use self::table::STATIC_TABLE;

use crate::field::FieldLine;

const STATIC_TABLE_LEN: usize = STATIC_TABLE.len();

type BothMatch = bool;
type Index = usize;

pub fn find<F: FieldLine>(line: &F) -> Option<(BothMatch, Index)> {
    let mut matched = None;

    for (idx, (name, value)) in STATIC_TABLE.iter().enumerate() {
        if line.name().eq_ignore_ascii_case(name) {
            // empty fields are unique
            if value.is_empty() {
                return Some((false, idx));
            }

            if line.value() == *value {
                return Some((true, idx));
            }

            matched = Some((false, idx));
        }
    }

    matched
}

pub fn get(index: usize) -> Option<(&'static [u8], &'static [u8])> {
    if index < STATIC_TABLE_LEN {
        Some(STATIC_TABLE[index])
    } else {
        None
    }
}
