use std::fmt;
use serde::{Deserialize, Serialize};

/// Variable-length byte array for EVM
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    /// Create a new Bytes from a Vec<u8>
    pub fn new(data: Vec<u8>) -> Self {
        Bytes(data)
    }

    /// Create empty Bytes
    pub fn empty() -> Self {
        Bytes(Vec::new())
    }

    /// Get the length of the bytes
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if bytes is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the bytes as a slice
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    /// Get the bytes as a mutable slice
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.0
    }

    /// Create from hex string
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let hex = hex.strip_prefix("0x").unwrap_or(hex);
        if hex.len() % 2 != 0 {
            return Err("Hex string must have even length".to_string());
        }
        
        let bytes = hex::decode(hex).map_err(|_| "Invalid hex")?;
        Ok(Bytes(bytes))
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        format!("0x{}", hex::encode(&self.0))
    }

    /// Pad to a specific length with zeros
    pub fn pad_to(&self, length: usize) -> Self {
        let mut result = self.0.clone();
        if result.len() < length {
            result.resize(length, 0);
        }
        Bytes(result)
    }

    /// Truncate to a specific length
    pub fn truncate_to(&self, length: usize) -> Self {
        let mut result = self.0.clone();
        if result.len() > length {
            result.truncate(length);
        }
        Bytes(result)
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl Default for Bytes {
    fn default() -> Self {
        Bytes::empty()
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(data: Vec<u8>) -> Self {
        Bytes(data)
    }
}

impl From<&[u8]> for Bytes {
    fn from(data: &[u8]) -> Self {
        Bytes(data.to_vec())
    }
}

impl From<String> for Bytes {
    fn from(data: String) -> Self {
        Bytes(data.into_bytes())
    }
}

impl From<&str> for Bytes {
    fn from(data: &str) -> Self {
        Bytes(data.as_bytes().to_vec())
    }
}
