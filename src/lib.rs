#[cfg(any(feature = "encode", feature = "decode"))]
use num_bigint::{BigInt, Sign};
#[cfg(feature = "pyo3")]
use pyo3::prelude::{
    Bound, PyModule, PyModuleMethods, PyRef, PyRefMut, PyResult, pyclass, pyfunction, pymethods,
    pymodule, wrap_pyfunction,
};
#[cfg(feature = "encode")]
use rand::prelude::Rng;
#[cfg(any(feature = "encode", feature = "decode"))]
use rust_embed::Embed;
#[cfg(any(feature = "encode", feature = "decode"))]
use std::collections::HashMap;
#[cfg(any(feature = "encode", feature = "decode"))]
use std::string::String;
#[cfg(any(feature = "encode", feature = "decode"))]
use std::sync::LazyLock;
#[cfg(feature = "pyo3")]
use std::sync::{Arc, Mutex};

#[cfg(any(feature = "encode", feature = "decode"))]
#[derive(Embed)]
#[folder = "千字文"]
#[include = "*.txt"]
struct QianZiWenAssets;

#[cfg(any(feature = "encode", feature = "decode"))]
static QIAN_ZI_WEN: LazyLock<(Vec<Vec<char>>, HashMap<char, Vec<String>>)> = LazyLock::new(|| {
    let mut character_matrix = vec![Vec::new(); 1000];
    let mut character_indexes: HashMap<char, Vec<String>> = HashMap::new();
    let mut temp_index: String;
    for file in QianZiWenAssets::iter() {
        for (index, character) in std::str::from_utf8(&QianZiWenAssets::get(&file).unwrap().data)
            .unwrap()
            .chars()
            .filter(|c| !c.is_whitespace())
            .enumerate()
        {
            if !character_matrix[index].contains(&character) {
                character_matrix[index].push(character);
            }
            temp_index = format!("{:03}", index);
            if !character_indexes.contains_key(&character) {
                character_indexes.insert(character, Vec::new());
            }
            character_indexes.entry(character).and_modify(|x| {
                if !x.contains(&temp_index) {
                    x.push(temp_index)
                }
            });
        }
    }
    character_indexes.shrink_to_fit();
    return (character_matrix, character_indexes);
});

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

#[cfg(all(feature = "pyo3", feature = "decode"))]
#[pyfunction(name = "encode")]
fn py_encode(text: String) -> PyResult<String> {
    Ok(encode(text))
}

#[cfg(all(feature = "pyo3", feature = "decode"))]
#[pyclass]
struct DecodeIterator {
    iter: Arc<Mutex<Box<dyn Iterator<Item = String> + Send>>>,
}

#[cfg(all(feature = "pyo3", feature = "decode"))]
#[pymethods]
impl DecodeIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(slf: PyRefMut<'_, Self>) -> Option<String> {
        slf.iter.lock().unwrap().next()
    }
}

#[cfg(all(feature = "pyo3", feature = "decode"))]
#[pyfunction(name = "decode")]
fn py_decode(text: String) -> PyResult<DecodeIterator> {
    Ok(DecodeIterator {
        iter: Arc::new(Mutex::new(Box::new(decode(text)))),
    })
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
