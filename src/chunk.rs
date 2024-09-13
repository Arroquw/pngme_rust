use crate::chunk_type::ChunkType;
use crc::{Crc, CRC_32_ISO_HDLC};
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Chunk {
    len: u32,
    chunktype: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

const CRC_PNG: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

#[allow(dead_code)]
impl Chunk {
    pub fn new(chunktype: ChunkType, data: Vec<u8>) -> Self {
        Self {
            len: data.len() as u32,
            chunktype: chunktype.clone(),
            data: data.clone(),
            crc: CRC_PNG.checksum(&[&chunktype.bytes(), data.as_slice()].concat()),
        }
    }

    /// The length of the data portion of this chunk.
    pub fn length(&self) -> u32 {
        self.len
    }

    /// The `ChunkType` of this chunk
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunktype
    }

    /// The raw data contained in this chunk in bytes
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// The CRC of this chunk
    pub fn crc(&self) -> u32 {
        self.crc
    }

    /// Returns the data stored in this chunk as a `String`. This function will return an error
    /// if the stored data is not valid UTF-8.
    pub fn data_as_string(&self) -> Result<String, ()> {
        Ok(String::from_utf8(self.data.clone()).unwrap())
    }

    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        [
            self.len.to_be_bytes().to_vec(),
            vec![
                self.chunktype.bytes()[0],
                self.chunktype.bytes()[1],
                self.chunktype.bytes()[2],
                self.chunktype.bytes()[3],
            ],
            self.data.clone(),
            self.crc.to_be_bytes().to_vec(),
        ]
        .concat()
    }
}

/// Something went wrong while decoding a chunk.
#[derive(Debug)]
pub struct ChunkDecodingError {
    /// The reason that decoding went wrong.
    reason: String,
}
impl ChunkDecodingError {
    fn boxed(reason: String) -> Box<Self> {
        Box::new(Self { reason })
    }
}

impl fmt::Display for ChunkDecodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bad chunk: {}", self.reason)
    }
}
impl Error for ChunkDecodingError {}

impl TryFrom<&[u8]> for Chunk {
    type Error = crate::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let mut c = Self {
            len: u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
            chunktype: ChunkType::try_from([bytes[4], bytes[5], bytes[6], bytes[7]])?,
            data: vec![],
            crc: 0,
        };

        let mut data: Vec<u8> = vec![];
        for (idx, b) in bytes.iter().enumerate() {
            if idx >= 8 {
                data.push(*b);
            }
        }
        let crc: [u8; 4] = data[data.len() - 4..data.len()].try_into().unwrap();
        (0..4).for_each(|_a| {
            data.pop();
        });
        if c.len != data.len() as u32 {
            println!(
                "Warning: lengths mismatch actual len: {} got len: {}",
                c.len,
                data.len()
            );
            c.len = data.len() as u32;
        }
        c.data = data.clone();
        let true_crc = CRC_PNG.checksum(&[&c.chunktype.bytes(), data.as_slice()].concat());
        c.crc = u32::from_be_bytes(crc);
        if c.crc != true_crc {
            return Err(ChunkDecodingError::boxed(format!(
                "Bad CRC (received {:04x}, expected {:04x})",
                c.crc, true_crc
            )));
        }

        Ok(c)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
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
        let chunk_string = chunk.data_as_string().unwrap();
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

        let chunk_string = chunk.data_as_string().unwrap();
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
