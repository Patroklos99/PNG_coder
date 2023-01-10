extern crate core;

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
}
