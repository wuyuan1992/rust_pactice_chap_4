// Write a function called swap_chars that takes two mutable string slices as parameters
// and swaps the first characters of each slice.
// If either of the slices is empty, the function should return None.
// If the slices have different lengths, the function should return None.
// If both slices are non-empty and have the same length,
// the function should swap the first characters of each slice
// and return Some(&str) with the modified first slice.

use std::str;

fn main() {
    let mut s1 = String::from("hello");
    let mut s2 = String::from("world");

    assert_eq!(swap_chars(&mut s1, &mut s2), Some("wello".to_string()));

    let mut s3 = String::from("goodbye");
    let mut s4 = String::from("");

    assert_eq!(swap_chars(&mut s3, &mut s4), None);

    let mut s5 = String::from("rust");
    let mut s6 = String::from("is fun");

    assert_eq!(swap_chars(&mut s5, &mut s6), None);
}

fn swap_chars(str1: &mut str, str2: &mut str) -> Option<String> {
    if str1.len() != str2.len() || str1.is_empty() {
        return None;
    }

    let mut result = String::new();
    result.push_str(&str2[..1]);
    result.push_str(&str1[1..]);

    Some(result)
}
