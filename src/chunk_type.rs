use std::cmp::{Eq, PartialEq};
use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct ChunkType {
    chunk: [u8; 4],
}

impl ChunkType {
    pub fn new(chunk: [u8; 4]) -> ChunkType {
        ChunkType { chunk }
    }

    fn bytes(&self) -> [u8; 4] {
        self.chunk
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.chunk[2].is_ascii_uppercase()
    }

    fn is_valid(&self) -> bool {
        let condition1 = self.is_reserved_bit_valid();
        let condition2 = self.is_alphabetic();
        condition1 && condition2
    }

    fn is_critical(&self) -> bool {
        self.chunk[0].is_ascii_uppercase()
    }

    fn is_public(&self) -> bool {
        self.chunk[1].is_ascii_uppercase()
    }

    fn is_safe_to_copy(&self) -> bool {
        self.chunk[3].is_ascii_lowercase()
    }

    fn is_alphabetic(&self) -> bool {
        for byte in self.chunk {
            if !byte.is_ascii_alphabetic() {
                return false;
            }
        }
        true
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        let new_chunk = ChunkType::new(value);
        if new_chunk.is_alphabetic() {
            Ok(new_chunk)
        } else {
            Err("Chunk isn't valid".into())
        }
    }
}
impl Eq for ChunkType {}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        for (byte, other) in self.chunk.iter().zip(other.chunk.iter()) {
            if byte != other {
                return false;
            }
        }
        true
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let new_chunk: &[u8] = s.as_bytes();
        let new_chunk: &[u8; 4] = new_chunk.try_into().unwrap();
        let new_chunk: ChunkType = ChunkType::new(*new_chunk);
        if new_chunk.is_alphabetic() {
            Ok(new_chunk)
        } else {
            Err("Chunk is invalid".into())
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = String::from_utf8(self.chunk.to_vec()).unwrap();
        write!(f, "{string}")
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

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
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
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
