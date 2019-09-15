extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    UnicodeSegmentation::graphemes(input, true).rev().collect()
}

/*works fine bu does not handle graphemes
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

str.chars()
returns an iterable on a String characters

iter.rev()
reverse itrator's direction

iter.collect()
Transforms an iterator into a collection

alt:
::<String>
"turbofish" helps the inference system deduce the type to collect to
not mandatory here since we have the return type of the function
*/