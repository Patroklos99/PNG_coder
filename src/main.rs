use std::str::FromStr;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

extern crate core;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let x = ChunkType {
        chunk_t: [82, 82, 82, 82]
    };
    println!("{}", x);
    let chunk_type = ChunkType::from_str("RuSt").unwrap();
    let data = "This is where your secret message will be!".as_bytes().to_vec();
    let chunk = Chunk::new(chunk_type, data);
    println!("{}", chunk)
}
