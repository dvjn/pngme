use fehler::{throw, throws};
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChunkTypeParseError {
    #[error("invalid length `{0}`, expected length `4`")]
    InvalidLength(usize),

    #[error("not valid utf8")]
    InvalidUtf8(#[from] std::str::Utf8Error),

    #[error("invalid character `{0}`, only upper and lower case alphabets allowed")]
    InvalidCharacter(char),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    #[throws(ChunkTypeParseError)]
    fn validate_content(content: &[u8]) {
        if content.len() != 4 {
            throw!(ChunkTypeParseError::InvalidLength(content.len()));
        }

        match std::str::from_utf8(content) {
            Err(error) => throw!(ChunkTypeParseError::InvalidUtf8(error)),
            Ok(content) => {
                for c in content.chars() {
                    if !(c.is_ascii() && c.is_alphabetic()) {
                        throw!(ChunkTypeParseError::InvalidCharacter(c))
                    }
                }
            }
        }
    }

    fn is_5th_bit_set(byte: &u8) -> bool {
        byte & 0b00100000 > 0
    }

    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        !ChunkType::is_5th_bit_set(&self.0[0])
    }

    pub fn is_public(&self) -> bool {
        !ChunkType::is_5th_bit_set(&self.0[1])
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        !ChunkType::is_5th_bit_set(&self.0[2])
    }

    pub fn is_safe_to_copy(&self) -> bool {
        ChunkType::is_5th_bit_set(&self.0[3])
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeParseError;

    #[throws(Self::Error)]
    fn try_from(value: [u8; 4]) -> Self {
        ChunkType::validate_content(&value)?;

        Self(value)
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeParseError;

    #[throws(Self::Err)]
    fn from_str(s: &str) -> Self {
        let value = s.as_bytes();

        ChunkType::validate_content(value)?;

        Self([value[0], value[1], value[2], value[3]])
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#}",
            std::str::from_utf8(&self.0).expect("checked in constructors for valid utf8")
        )
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
