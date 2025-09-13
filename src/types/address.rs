use std::fmt;
use serde::{Deserialize, Serialize};

/// Ethereum address (20 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(pub [u8; 20]);

impl Address {
    /// Create a new address from a 20-byte array
    pub fn new(bytes: [u8; 20]) -> Self {
        Address(bytes)
    }

    /// Create a zero address
    pub fn zero() -> Self {
        Address([0u8; 20])
    }

    /// Get the address as a byte array
    pub fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }

    /// Create address from hex string
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let hex = hex.strip_prefix("0x").unwrap_or(hex);
        if hex.len() != 40 {
            return Err("Address must be 40 hex characters".to_string());
        }
        
        let mut bytes = [0u8; 20];
        for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
            let byte_str = std::str::from_utf8(chunk).map_err(|_| "Invalid hex")?;
            bytes[i] = u8::from_str_radix(byte_str, 16).map_err(|_| "Invalid hex")?;
        }
        
        Ok(Address(bytes))
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        format!("0x{}", hex::encode(self.0))
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl Default for Address {
    fn default() -> Self {
        Address::zero()
    }
}
