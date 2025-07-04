#![cfg(feature = "encode")]
use crate::QIAN_ZI_WEN;
use num_bigint::BigInt;
use num_bigint::Sign;
use rand::prelude::Rng;
use std::string::String;

/// Encodes the given text into a string using the "Thousand Character Classic" character matrix.
///
/// # Arguments
/// * `text` - The input text to encode.
///
/// # Returns
/// A string representing the encoded text.
///
/// # Features
/// This function requires the `encode` feature to be enabled.
///
/// # Examples
/// ```
/// use base1000::encode;
///
/// fn main() {
///     let encoded = encode("114514".to_string());
///     println!("{}", encoded);
/// }
/// ```
///
/// # Python Bindings
/// This function is exposed to Python as `base1000.encode`.
/// Requires the `pyo3` feature to be enabled for Python bindings.
///
/// # Python Example
/// ```python
/// from base1000 import base1000
///
/// encoded = base1000.encode("114514")
/// print(encoded)
/// ```
pub fn encode(text: String) -> String {
    let mut rng = rand::rng();
    return encode_with_rng(text, &mut rng);
}

fn encode_with_rng(text: String, rng: &mut impl Rng) -> String {
    if text.is_empty() {
        return text;
    }
    let character_matrix = &*QIAN_ZI_WEN.0;
    let mut bigint_string: String = BigInt::from_bytes_be(Sign::Plus, text.as_bytes()).to_string();
    bigint_string = "0".repeat(3 - bigint_string.len() % 3) + &bigint_string;
    return bigint_string
        .chars()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|chunk| {
            let index = chunk.iter().collect::<String>().parse::<usize>().unwrap();
            character_matrix[index][rng.random_range(..character_matrix[index].len())]
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::prelude::*;

    #[test]
    fn test_encode() {
        let text = String::from("Hello, world!");
        let encoded = encode(text.clone());
        assert!(!encoded.is_empty());
        assert_ne!(encoded, text);
    }

    #[test]
    fn test_encode_deterministic() {
        let text = String::from("114514");
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);
        let encoded1 = encode_with_rng(text.clone(), &mut rng1);
        let encoded2 = encode_with_rng(text.clone(), &mut rng2);
        assert_eq!(encoded1, encoded2);
        assert_eq!(encoded1, "夜裳移柰梧");
    }
}
