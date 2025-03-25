use itertools::Itertools;
use num_bigint::{BigInt, Sign};
use pyo3::prelude::{
    Bound, PyModule, PyModuleMethods, PyResult, pyfunction, pymodule, wrap_pyfunction,
};
use rand::prelude::Rng;
use rust_embed::Embed;
use std::collections::HashMap;
use std::string::String;
use std::sync::LazyLock;

#[derive(Embed)]
#[folder = "千字文"]
#[include = "*.txt"]
struct Asset;

static QZW: LazyLock<(Vec<Vec<char>>, HashMap<char, Vec<String>>)> = LazyLock::new(|| {
    let mut result = vec![Vec::new(); 1000];
    let mut indexes: HashMap<char, Vec<String>> = HashMap::new();
    let mut temp: String;
    for file in Asset::iter() {
        for (index, item) in std::str::from_utf8(&Asset::get(&file).unwrap().data)
            .unwrap()
            .chars()
            .filter(|c| !c.is_whitespace())
            .enumerate()
        {
            if !result[index].contains(&item) {
                result[index].push(item);
            }
            temp = format!("{:03}", index);
            if !indexes.contains_key(&item) {
                indexes.insert(item, Vec::new());
            }
            indexes.entry(item).and_modify(|x| {
                if !x.contains(&temp) {
                    x.push(temp)
                }
            });
        }
    }
    indexes.shrink_to_fit();
    return (result, indexes);
});

pub fn encode(text: String) -> String {
    let mut rng = rand::rng();
    return encode_with_rng(text, &mut rng);
}

fn encode_with_rng(text: String, rng: &mut impl Rng) -> String {
    let qzw = &*QZW.0;
    let mut tmp: String = BigInt::from_bytes_be(Sign::Plus, text.as_bytes()).to_string();
    tmp = "0".repeat((3 - tmp.len() % 3) % 3) + &tmp;
    return tmp
        .chars()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|chunk| {
            let index = chunk.iter().collect::<String>().parse::<usize>().unwrap();
            qzw[index][rng.random_range(..qzw[index].len())]
        })
        .collect();
}

pub fn decode(text: String) -> impl Iterator<Item = String> {
    let qzw = &QZW.1;
    return text
        .chars()
        .filter_map(|x| qzw.get(&x).cloned())
        .multi_cartesian_product()
        .filter_map(|item| {
            BigInt::parse_bytes(item.join("").as_bytes(), 10)
                .and_then(|bigint| String::from_utf8(bigint.to_bytes_be().1).ok())
        });
}

#[pyfunction(name = "encode")]
fn py_encode(text: String) -> PyResult<String> {
    Ok(encode(text))
}

#[pyfunction(name = "decode")]
fn py_decode(text: String) -> PyResult<Vec<String>> {
    Ok(decode(text).collect::<Vec<String>>())
}

#[pymodule]
fn base1000(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_encode, m)?)?;
    m.add_function(wrap_pyfunction!(py_decode, m)?)?;
    //    m.add_class::<py_decode<_>>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let text = String::from("Hello, world!");
        let encoded = encode(text.clone());
        assert!(!encoded.is_empty());
        assert_ne!(encoded, text);
    }

    #[test]
    fn test_qzw_initialization() {
        let qzw = &*QZW.0;
        assert!(!qzw.is_empty());
        assert_eq!(qzw.len(), 1000);
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
