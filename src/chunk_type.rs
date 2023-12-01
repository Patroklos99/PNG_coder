#![allow(unused_variables)]

use std::fmt;
use std::fmt::{format, Formatter, write};
use std::ops::{Add, Not};
use std::str::FromStr;
use std::thread::current;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    pub(crate) chunk_t: [u8; 4],
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseChunkError;

impl ChunkType {
    fn new(chunk: String) -> Self {
        let char1: Vec<u8> = chunk.as_bytes().to_vec();
        Self {
            chunk_t: [char1[0], char1[1], char1[2], char1[3]]
        }
    }

    pub fn bytes(&self) -> [u8; 4] {
        self.chunk_t
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    //validate 5th bit of the first indexed charactere is Critical (0 (uppercase) = critical)
    pub(crate) fn is_critical(&self) -> bool {
        let x = format!("0{:b}", self.bytes()[0]);
        x.chars().nth(x.len() - 6).unwrap() as u8 - '0' as u8 == 0
    }

    pub(crate) fn is_public(&self) -> bool {
        let x = format!("0{:b}", self.bytes()[1]);
        x.chars().nth(x.len() - 6).unwrap() as u8 - '0' as u8 == 0
        // let my_int :u32 = x.parse().unwrap();
        // my_int & (1 << 5) == 0
    }

    pub(crate) fn is_reserved_bit_valid(&self) -> bool {
        let x = format!("0{:b}", self.bytes()[2]);
        x.chars().nth(x.len() - 6).unwrap() as u8 - '0' as u8 == 0
    }

    pub(crate) fn is_safe_to_copy(&self) -> bool {
        let x = format!("0{:b}", self.bytes()[3]);
        x.chars().nth(x.len() - 6).unwrap() as u8 - '0' as u8 == 1
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.len() != 4 {
            Err("the length of the chunk has to be 4")
        } else {
            Ok(ChunkType { chunk_t: value })
        }
    }
}

impl FromStr for ChunkType {
    type Err = ParseChunkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.chars().all(|c| c.is_alphabetic());
        let char1: Vec<u8> = s.as_bytes().to_vec();
        if x == true {
            Ok(ChunkType { chunk_t: [char1[0], char1[1], char1[2], char1[3]] })
        } else {
            Err(ParseChunkError)
        }
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8(Vec::from(self.chunk_t)).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        std::assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        std::assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }


    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        std::assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = std::format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
