use std::fmt;
use std::fmt::{Formatter, write};
use std::str::FromStr;
use std::thread::current;


#[derive(Debug, PartialEq, Eq)]
struct ChunkType {
    chunk_t : [u8; 4],
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseChunkError;

impl ChunkType {
    fn new(chunk : String) -> Self {
        Self {
            chunk_t : [chunk[0], chunk[1], chunk[2], chunk[3]]
        }
    }

    fn bytes(&self) -> [u8;4] {
        self.chunk_t
    }

    fn is_valid(&self) -> bool {
        self.chunk_t.len() == 4
    }

    fn is_critical(&self) -> bool {
        self.chunk_t[0] == 0
    }

    fn is_public(&self) -> bool {
        self.chunk_t[1] == 0
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.chunk_t[2] == 0
    }

    fn is_safe_to_copy(&self) -> bool {
        self.chunk_t[3] == 1
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.len() != 4 {
            Err("the length of the chunk has to be 4")
        } else {
            Ok(ChunkType { chunk_t: value})
        }
    }
}

impl FromStr for ChunkType {
    type Err = (ParseChunkError);

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunk_t = s.parse::<String>().map_err(|_| ParseChunkError)?;
        Ok(chunk_t.parse()?)
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.chunk_t[0], self.chunk_t[1], self.chunk_t[2], self.chunk_t[3])
    }
}