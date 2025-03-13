use num_bigint::{BigInt, Sign};
use rand::prelude::*;
use rust_embed::Embed;
use std::string::String;
use std::sync::LazyLock;

#[derive(Embed)]
#[folder = "千字文"]
#[include = "*.txt"]
struct Asset;

static QZW: LazyLock<Vec<Vec<char>>> = LazyLock::new(|| {
    let mut result = vec![Vec::new(); 1000];
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
        }
    }
    return result;
});

pub fn encode(text: String) -> String {
    let mut rng = rand::rng();
    let qzw = &*QZW;
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

pub fn decode(_text: String) {
    let qzw = &*QZW;
    dbg!(qzw);
}
