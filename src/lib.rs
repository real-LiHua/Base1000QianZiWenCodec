//! # Base1000 Thousand Character Classic Encoder
//!
//! Base1000 is a text encoder based on the "Thousand Character Classic",
//! supporting encoding any text into a sequence of "Thousand Character Classic" characters
//! and decoding it back to the original text.
use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(any(feature = "encode", feature = "decode"))] {
        use num_bigint::BigInt;
        use rust_embed::Embed;
        use std::collections::HashMap;
        use std::string::String;
        use std::sync::LazyLock;
    }
}

cfg_if! {
    if #[cfg(feature = "encode")] {
        use num_bigint::Sign;
        use rand::prelude::Rng;
    }
}

#[cfg(feature = "decode")]
use itertools::Itertools;

#[cfg(feature = "pyo3")]
use pyo3::prelude::{Bound, PyModule, PyResult, pymodule};

#[cfg(all(feature = "pyo3", any(feature = "encode", feature = "decode")))]
use pyo3::prelude::{PyModuleMethods, wrap_pyfunction};

#[cfg(all(feature = "pyo3", feature = "decode"))]
use pyo3::prelude::{PyRef, PyRefMut, pyclass, pyfunction, pymethods};

#[cfg(all(feature = "pyo3", feature = "decode"))]
use std::sync::{Arc, Mutex};

#[cfg(any(feature = "encode", feature = "decode"))]
#[derive(Embed)]
#[folder = "千字文"]
#[include = "*.txt"]
struct QianZiWenAssets;

#[cfg(any(feature = "encode", feature = "decode"))]
static QIAN_ZI_WEN: LazyLock<(Vec<Vec<char>>, HashMap<char, Vec<String>>)> = LazyLock::new(|| {
    #[cfg(feature = "encode")]
    let mut character_matrix = vec![Vec::new(); 1000];
    #[cfg(not(feature = "encode"))]
    let character_matrix = vec![Vec::new(); 1000];

    #[cfg(feature = "decode")]
    let mut character_indexes: HashMap<char, Vec<String>> = HashMap::new();
    #[cfg(not(feature = "decode"))]
    let character_indexes: HashMap<char, Vec<String>> = HashMap::new();

    for file in QianZiWenAssets::iter() {
        for (index, character) in std::str::from_utf8(&QianZiWenAssets::get(&file).unwrap().data)
            .unwrap()
            .chars()
            .filter(|c| !c.is_whitespace())
            .enumerate()
        {
            #[cfg(feature = "encode")]
            if !character_matrix[index].contains(&character) {
                character_matrix[index].push(character);
            }

            #[cfg(feature = "decode")]
            if !character_indexes.contains_key(&character) {
                character_indexes.insert(character, Vec::new());
            }

            #[cfg(feature = "decode")]
            character_indexes.entry(character).and_modify(|x| {
                let temp_index = format!("{:03}", index);
                if !x.contains(&temp_index) {
                    x.push(temp_index)
                }
            });
        }
    }

    #[cfg(feature = "decode")]
    character_indexes.shrink_to_fit();

    return (character_matrix, character_indexes);
});

/// Encodes the given text into a string using the "Thousand Character Classic" character matrix.
///
/// # Arguments
/// * `text` - The input text to encode.
///
/// # Returns
/// A string representing the encoded text.
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
#[cfg(feature = "encode")]
pub fn encode(text: String) -> String {
    let mut rng = rand::rng();
    return encode_with_rng(text, &mut rng);
}

#[cfg(feature = "encode")]
fn encode_with_rng(text: String, rng: &mut impl Rng) -> String {
    if text.is_empty() {
        return text;
    }
    let character_matrix = &*QIAN_ZI_WEN.0;
    let mut bigint_string: String = BigInt::from_bytes_be(Sign::Plus, text.as_bytes()).to_string();
    bigint_string = "0".repeat((3 - bigint_string.len() % 3) % 3) + &bigint_string;
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

/// Decodes the given text into an iterator of possible original strings.
///
/// # Arguments
/// * `text` - The encoded text to decode.
///
/// # Returns
/// An iterator over possible decoded strings.
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
#[cfg(feature = "decode")]
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

#[cfg(all(feature = "pyo3", feature = "encode"))]
#[pyfunction(name = "encode")]
fn py_encode(text: String) -> PyResult<String> {
    Ok(encode(text))
}

cfg_if! {
    if #[cfg(all(feature = "pyo3", feature = "decode"))] {
        #[pyclass]
        struct DecodeIterator {
            iter: Arc<Mutex<Box<dyn Iterator<Item = String> + Send>>>,
        }

        #[pymethods]
        impl DecodeIterator {
            fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
                slf
            }
            fn __next__(slf: PyRefMut<'_, Self>) -> Option<String> {
                slf.iter.lock().unwrap().next()
            }
        }

        #[pyfunction(name = "decode")]
        fn py_decode(text: String) -> PyResult<DecodeIterator> {
            Ok(DecodeIterator {
                iter: Arc::new(Mutex::new(Box::new(decode(text)))),
            })
        }
    }
}

#[cfg(feature = "pyo3")]
#[pymodule]
fn base1000(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(feature = "encode")]
    m.add_function(wrap_pyfunction!(py_encode, m)?)?;
    #[cfg(feature = "decode")]
    m.add_function(wrap_pyfunction!(py_decode, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "encode")]
    use rand::prelude::*;

    #[cfg(feature = "encode")]
    #[test]
    fn test_encode() {
        let text = String::from("Hello, world!");
        let encoded = encode(text.clone());
        assert!(!encoded.is_empty());
        assert_ne!(encoded, text);
    }

    #[cfg(any(feature = "encode", feature = "decode"))]
    #[test]
    fn test_qzw_initialization() {
        let qzw = &*QIAN_ZI_WEN.0;
        assert!(!qzw.is_empty());
        assert_eq!(qzw.len(), 1000);
    }

    #[cfg(feature = "encode")]
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

    #[cfg(all(feature = "encode", feature = "decode"))]
    #[test]
    fn test_decode() {
        let text = String::from("Hello, world!");
        let encoded = encode(text.clone());
        let decoded: Vec<String> = decode(encoded).collect();
        assert!(decoded.contains(&text));
    }

    #[cfg(all(feature = "encode", feature = "decode"))]
    #[test]
    fn test_empty_input() {
        let text = String::from("");
        let encoded = encode(text.clone());
        assert!(encoded.is_empty());
        let decoded: Vec<String> = decode(encoded).collect();
        assert!(decoded.contains(&text));
    }

    #[cfg(feature = "decode")]
    #[test]
    fn test_invalid_character_in_decode() {
        let invalid_text = String::from("InvalidCharacters");
        let decoded: Vec<String> = decode(invalid_text).collect();
        assert!(decoded.is_empty());
    }
}
