extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation as UniSeg;

pub fn reverse(input: &str) -> String {
    if input.is_empty() {
        input.to_string();
    }

    UniSeg::graphemes(input, true)
        .rev()
        .collect()
}
