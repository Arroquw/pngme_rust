use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ChunkType {
    code: [u8; 4],
}
#[allow(dead_code)]
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.code
    }

    pub fn is_valid_byte(byte: u8) -> bool {
        (65..90).contains(&byte) || (97..122).contains(&byte)
    }

    pub fn is_valid(&self) -> bool {
        self.code[2] & (1 << 5) == 0
    }

    pub fn is_critical(&self) -> bool {
        println!("{}", self.code[0] & (1 << 4));
        (self.code[0] & (1 << 5)) == 0
    }

    pub fn is_public(&self) -> bool {
        (self.code[1] & (1 << 5)) == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        (self.code[2] & (1 << 5)) == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        (self.code[3] & (1 << 5)) != 0
    }
}
#[derive(Debug)]
pub struct PngDecodeError {
    reason: String,
}
impl PngDecodeError {
    fn boxed(reason: String) -> Box<Self> {
        Box::new(Self { reason })
    }
}

impl fmt::Display for PngDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bad PNG: {}", self.reason)
    }
}
impl Error for PngDecodeError {}

impl FromStr for ChunkType {
    type Err = &'static str;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut code: [u8; 4] = [0u8; 4];
        for (i, &byte) in str.as_bytes().iter().enumerate() {
            if !ChunkType::is_valid_byte(byte) {
                return Err("Invalid byte in chunk type string");
            }
            code[i] = byte;
        }
        let chunktype = ChunkType { code };
        Ok(chunktype)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let chunktype = ChunkType { code: value };
        if chunktype.is_valid() {
            Ok(chunktype)
        } else {
            Err(PngDecodeError::boxed(format!(
                "Bad data type! (received {:?})",
                value
            )))
        }
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            char::from(self.code[0]),
            char::from(self.code[1]),
            char::from(self.code[2]),
            char::from(self.code[3])
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
