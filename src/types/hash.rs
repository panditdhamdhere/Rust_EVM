use std::fmt;
use serde::{Deserialize, Serialize};

/// Ethereum hash (32 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash(pub [u8; 32]);

impl Hash {
    /// Create a new hash from a 32-byte array
    pub fn new(bytes: [u8; 32]) -> Self {
        Hash(bytes)
    }

    /// Create a zero hash
    pub fn zero() -> Self {
        Hash([0u8; 32])
    }

    /// Get the hash as a byte array
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Create hash from hex string
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let hex = hex.strip_prefix("0x").unwrap_or(hex);
        if hex.len() != 64 {
            return Err("Hash must be 64 hex characters".to_string());
        }
        
        let mut bytes = [0u8; 32];
        for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
            let byte_str = std::str::from_utf8(chunk).map_err(|_| "Invalid hex")?;
            bytes[i] = u8::from_str_radix(byte_str, 16).map_err(|_| "Invalid hex")?;
        }
        
        Ok(Hash(bytes))
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        format!("0x{}", hex::encode(self.0))
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl Default for Hash {
    fn default() -> Self {
        Hash::zero()
    }
}
