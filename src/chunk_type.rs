use std::{
  fmt::Display,
  io::{Error as IoError, ErrorKind},
  str::FromStr,
};

use crate::Error;

#[derive(Debug, PartialEq, Eq)]
struct ChunkType {
  value: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
  type Error = Error;
  fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
    Ok(ChunkType { value })
  }
}

impl FromStr for ChunkType {
  type Err = crate::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if !s.chars().all(|i| i.is_ascii_alphabetic()) {
      return Err(Box::new(IoError::new(
        ErrorKind::InvalidInput,
        "only ascii alphabets",
      )));
    }
    let mut value = [0u8; 4];
    value.copy_from_slice(&s.as_bytes()[..4]);

    Ok(ChunkType { value })
  }
}

impl Display for ChunkType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", String::from_utf8_lossy(&self.value))
  }
}

// impl PartialEq for ChunkType {
//   fn eq(&self, other: &Self) -> bool {
//     self.value == other.value
//   }
// }

// impl Eq for ChunkType {}

impl ChunkType {
  fn bytes(&self) -> [u8; 4] {
    self.value
  }

  fn is_valid(&self) -> bool {
    // self.value[2] >= b'A' && self.value[2] <= b'Z'
    u8::is_ascii_uppercase(&self.value[2]) && self.value.iter().all(|i| i.is_ascii_alphabetic())
  }

  fn is_critical(&self) -> bool {
    // self.value[0] >= b'A' && self.value[0] <= b'Z'

    u8::is_ascii_uppercase(&self.value[0])
  }

  fn is_public(&self) -> bool {
    // self.value[1] >= b'A' && self.value[1] <= b'Z'

    u8::is_ascii_uppercase(&self.value[1])
  }

  fn is_reserved_bit_valid(&self) -> bool {
    // self.value[2] >= b'A' && self.value[2] <= b'Z'

    u8::is_ascii_uppercase(&self.value[2])
  }

  fn is_safe_to_copy(&self) -> bool {
    // !(self.value[3] >= b'A' && self.value[3] <= b'Z')

    !u8::is_ascii_uppercase(&self.value[3])
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
