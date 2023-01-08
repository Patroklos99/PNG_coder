extern crate core;

use crate::chunk::ChunkType;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let x = ChunkType {
        chunk_t : [82, 82 , 82 , 82]
    };
    println!("{}", x);
}
