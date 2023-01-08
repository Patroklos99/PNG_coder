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
    let st = String::from("string");
    let vec_char : Vec<char> = st.chars().collect();
    println!("{:?}", vec_char);
    let x = 0b00010000;
    println!("x is {} in decimal, {:b} in binary", x, x);
    for i in 0..8 {
        println!("Bit {} is set? {}", i, x & (1 << i) != 0);
    }
}
