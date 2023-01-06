use crate::chunk::ChunkType;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;



fn main() {
    let chunk_1 = ChunkType {
        chunk_t: [1, 3, 4, 5],
    };
    println!("{}", chunk_1)
}
