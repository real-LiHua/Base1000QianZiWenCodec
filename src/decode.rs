#![cfg(feature = "decode")]
use crate::QIAN_ZI_WEN;
use itertools::Itertools;
use num_bigint::BigInt;
use std::string::String;

/// Decodes the given text into an iterator of possible original strings.
///
/// # Arguments
/// * `text` - The encoded text to decode.
///
/// # Returns
/// An iterator over possible decoded strings.
///
/// # Features
/// This function requires the `decode` feature to be enabled.
///
/// # Examples
/// ```
/// use base1000::decode;
///
/// fn main() {
///     for decoded in decode("夜裳移柰梧".to_string()) {
///         println!("{}", decoded);
///     }
/// }
/// ```
///
/// # Python Bindings
/// This function is exposed to Python as `base1000.decode`.
/// Requires the `pyo3` feature to be enabled for Python bindings.
///
/// # Python Example
/// ```python
/// import base1000
///
/// for decoded in base1000.decode("夜裳移柰梧"):
///     print(decoded)
/// ```
pub fn decode(text: String) -> impl Iterator<Item = String> {
    let character_indexes = &QIAN_ZI_WEN.1;
    return text
        .chars()
        .filter_map(|character| character_indexes.get(&character).cloned())
        .multi_cartesian_product()
        .filter_map(|item| {
            BigInt::parse_bytes(item.join("").as_bytes(), 10)
                .and_then(|bigint| String::from_utf8(bigint.to_bytes_be().1).ok())
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_character_in_decode() {
        let invalid_text = String::from("InvalidCharacters");
        let decoded: Vec<String> = decode(invalid_text).collect();
        assert!(decoded.is_empty());
    }
}
