use crc::{Crc, CRC_32_ISO_HDLC};
use fehler::{throw, throws};
use std::fmt::Display;
use thiserror::Error;

use crate::chunk_type::{ChunkType, ChunkTypeParseError};

#[derive(Debug, Error)]
pub enum ChunkParseError {
    #[error("chunk too short")]
    ChunkTooShort,

    #[error("invalid chunk type")]
    InvalidChunkType(#[from] ChunkTypeParseError),

    #[error("invalid crc value. expected `{expected}` but got `{actual}`")]
    InvalidCrc { expected: u32, actual: u32 },

    #[error("chunk too long")]
    ChunkTooLong,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc = Chunk::calculate_crc(&chunk_type, &data);

        Chunk {
            length: data.len() as u32,
            chunk_type,
            data,
            crc,
        }
    }

    pub fn length(&self) -> usize {
        self.length as usize
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> String {
        String::from_utf8_lossy(&self.data).to_string()
    }

    fn calculate_crc(chunk_type: &ChunkType, data: &[u8]) -> u32 {
        let hasher = Crc::<u32>::new(&CRC_32_ISO_HDLC);

        let mut digest = hasher.digest();
        digest.update(&chunk_type.bytes());
        digest.update(data);

        digest.finalize()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkParseError;

    #[throws(Self::Error)]
    fn try_from(raw_chunk: &[u8]) -> Self {
        let length: [u8; 4] = raw_chunk
            .get(..4)
            .ok_or(ChunkParseError::ChunkTooShort)?
            .try_into()
            .expect("slice of length 4");
        let length = u32::from_be_bytes(length);

        let chunk_type: [u8; 4] = raw_chunk
            .get(4..8)
            .ok_or(ChunkParseError::ChunkTooShort)?
            .try_into()
            .expect("slice of length 4");
        let chunk_type = ChunkType::try_from(chunk_type)?;

        let data_end_index = 8 + length as usize;

        let data = raw_chunk
            .get(8..8 + length as usize)
            .ok_or(ChunkParseError::ChunkTooShort)?
            .to_vec();

        let crc: [u8; 4] = raw_chunk
            .get(data_end_index..data_end_index + 4)
            .ok_or(ChunkParseError::ChunkTooShort)?
            .try_into()
            .expect("slice of length 4");
        let crc = u32::from_be_bytes(crc);

        let calculated_crc = Chunk::calculate_crc(&chunk_type, &data);
        if calculated_crc != crc {
            throw!(ChunkParseError::InvalidCrc {
                expected: crc,
                actual: calculated_crc
            })
        }

        if raw_chunk.len() > data_end_index + 4 {
            throw!(ChunkParseError::ChunkTooLong)
        }

        Chunk {
            length,
            chunk_type,
            data,
            crc,
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Chunk {{\n  length: {}\n  chunk_type: {}\n  data: {:?}\n  crc: {}\n}}\n",
            self.length(),
            self.chunk_type,
            self.data,
            self.crc()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
