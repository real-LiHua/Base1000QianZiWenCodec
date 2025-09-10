//! # Base1000 Thousand Character Classic Encoder
//!
//! Base1000 is a text encoder based on the "Thousand Character Classic",
//! supporting encoding any text into a sequence of "Thousand Character Classic" characters
//! and decoding it back to the original text.
//!
//! ## Features
//! - `encode`: Enables text encoding functionality.
//! - `decode`: Enables text decoding functionality.
//! - `pyo3`: Enables Python bindings for encoding and decoding.
use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(any(feature = "encode", feature = "decode"))] {
        use rust_embed::Embed;
        use std::collections::HashMap;
        use std::string::String;
        use std::sync::LazyLock;
    }
}

#[cfg(feature = "pyo3")]
use pyo3::prelude::{Bound, PyModule, PyResult, pymodule};

#[cfg(all(feature = "pyo3", any(feature = "encode", feature = "decode")))]
use pyo3::prelude::{PyModuleMethods, wrap_pyfunction};

#[cfg(all(feature = "pyo3", feature = "decode"))]
use pyo3::prelude::{PyRef, PyRefMut, pyclass, pyfunction, pymethods};

#[cfg(all(feature = "pyo3", feature = "decode"))]
use std::sync::{Arc, Mutex};

#[cfg(feature = "decode")]
mod decode;
#[cfg(feature = "encode")]
mod encode;

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
    let character_matrix = vec![Vec::new(); 0];

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

#[cfg(feature = "decode")]
pub use crate::decode::decode;
#[cfg(feature = "encode")]
pub use crate::encode::encode;

#[cfg(all(feature = "pyo3", feature = "encode"))]
#[pyfunction(name = "encode")]
#[pyo3(signature = (text: "str") -> "str")]
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
        #[pyo3(signature = (text: "str") -> "list[str]")]
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

    #[cfg(any(feature = "encode", feature = "decode"))]
    #[test]
    fn test_qzw_initialization() {
        let qzw = &*QIAN_ZI_WEN.0;
        assert!(!qzw.is_empty());
        assert_eq!(qzw.len(), 1000);
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
        assert!(decoded.is_empty());
    }
}
