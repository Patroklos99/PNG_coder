use std::str::FromStr;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

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