use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use base64::{engine::general_purpose, Engine};

pub fn load_tiktoken(file: &str) -> Vec<Vec<u8>> {
    let f = File::open(file).unwrap();
    let reader = BufReader::new(f);
    let mut count = 0;
    let mut pieces = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut s = line.trim().split(" ");
        let peice = s.next().unwrap();
        let id = s.next().unwrap();

        let peice = general_purpose::STANDARD.decode(peice.as_bytes()).unwrap();
        pieces.push(peice);
        let id: u32 = id.parse().unwrap();
        assert_eq!(id, count);
        count += 1;
    }
    pieces
}

pub fn encode(pieces: &[Vec<u8>], mut bytes: &[u8]) -> Vec<u32> {
    let mut res = Vec::new();

    while !bytes.is_empty() {
        let mut i: Option<usize> = None;
        for (id, piece) in pieces.iter().enumerate() {
            if bytes.starts_with(piece) && i.map(|i| pieces[i].len() < piece.len()).unwrap_or(true)
            {
                i = Some(id);
            }
        }

        let i = i.unwrap();
        res.push(i as u32);
        bytes = &bytes[pieces[i].len()..];
    }

    res
}

pub fn decode(pieces: &[Vec<u8>], ids: &[u32]) -> Vec<u8> {
    ids.iter()
        .map(|id| pieces[*id as usize].clone())
        .flatten()
        .collect()
}
