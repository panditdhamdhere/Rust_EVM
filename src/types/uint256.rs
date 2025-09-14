use std::fmt;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{Zero, One};
use serde::{Deserialize, Serialize};

/// 256-bit unsigned integer for EVM
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Uint256(pub BigUint);

impl Uint256 {
    /// Create a new Uint256 from a BigUint
    pub fn new(value: BigUint) -> Self {
        Uint256(value)
    }

    /// Create a zero Uint256
    pub fn zero() -> Self {
        Uint256(BigUint::zero())
    }

    /// Create a one Uint256
    pub fn one() -> Self {
        Uint256(BigUint::one())
    }

    /// Create from u64
    pub fn from_u64(value: u64) -> Self {
        Uint256(value.to_biguint().unwrap())
    }

    /// Create from u32
    pub fn from_u32(value: u32) -> Self {
        Uint256(value.to_biguint().unwrap())
    }

    /// Create from u8
    pub fn from_u8(value: u8) -> Self {
        Uint256(value.to_biguint().unwrap())
    }

    /// Create from byte array (big-endian)
    pub fn from_bytes_be(bytes: &[u8]) -> Self {
        Uint256(BigUint::from_bytes_be(bytes))
    }

    /// Create from byte array (little-endian)
    pub fn from_bytes_le(bytes: &[u8]) -> Self {
        Uint256(BigUint::from_bytes_le(bytes))
    }

    /// Convert to byte array (big-endian, 32 bytes)
    pub fn to_bytes_be(&self) -> [u8; 32] {
        let bytes = self.0.to_bytes_be();
        let mut result = [0u8; 32];
        let start = 32usize.saturating_sub(bytes.len());
        result[start..].copy_from_slice(&bytes);
        result
    }

    /// Convert to byte array (little-endian, 32 bytes)
    pub fn to_bytes_le(&self) -> [u8; 32] {
        let bytes = self.0.to_bytes_le();
        let mut result = [0u8; 32];
        let end = bytes.len().min(32);
        result[..end].copy_from_slice(&bytes[..end]);
        result
    }

    /// Check if the value is zero
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Check if the value is one
    pub fn is_one(&self) -> bool {
        self.0.is_one()
    }

    /// Get the underlying BigUint
    pub fn as_biguint(&self) -> &BigUint {
        &self.0
    }

    /// Convert to u64 (returns 0 if value is too large)
    pub fn to_u64(&self) -> u64 {
        self.0.to_u64_digits().first().copied().unwrap_or(0)
    }

    /// Convert to u32 (returns 0 if value is too large)
    pub fn to_u32(&self) -> u32 {
        self.0.to_u32_digits().first().copied().unwrap_or(0)
    }

    /// Convert to u8 (returns 0 if value is too large)
    pub fn to_u8(&self) -> u8 {
        self.0.to_u32_digits().first().map(|&x| x as u8).unwrap_or(0)
    }

    /// Safely convert to u64 with overflow check
    pub fn to_u64_safe(&self) -> Result<u64, String> {
        if self.0.bits() > 64 {
            Err("Value too large for u64".to_string())
        } else {
            Ok(self.to_u64())
        }
    }

    /// Safely convert to u32 with overflow check
    pub fn to_u32_safe(&self) -> Result<u32, String> {
        if self.0.bits() > 32 {
            Err("Value too large for u32".to_string())
        } else {
            Ok(self.to_u32())
        }
    }

    /// Safely convert to u8 with overflow check
    pub fn to_u8_safe(&self) -> Result<u8, String> {
        if self.0.bits() > 8 {
            Err("Value too large for u8".to_string())
        } else {
            Ok(self.to_u8())
        }
    }
}

impl std::ops::Add for Uint256 {
    type Output = Uint256;

    fn add(self, rhs: Self) -> Self::Output {
        Uint256(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Uint256 {
    type Output = Uint256;

    fn sub(self, rhs: Self) -> Self::Output {
        Uint256(self.0 - rhs.0)
    }
}

impl std::ops::Mul for Uint256 {
    type Output = Uint256;

    fn mul(self, rhs: Self) -> Self::Output {
        Uint256(self.0 * rhs.0)
    }
}

impl std::ops::Div for Uint256 {
    type Output = Uint256;

    fn div(self, rhs: Self) -> Self::Output {
        Uint256(self.0 / rhs.0)
    }
}

impl std::ops::Rem for Uint256 {
    type Output = Uint256;

    fn rem(self, rhs: Self) -> Self::Output {
        Uint256(self.0 % rhs.0)
    }
}

impl std::ops::BitAnd for Uint256 {
    type Output = Uint256;

    fn bitand(self, rhs: Self) -> Self::Output {
        Uint256(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for Uint256 {
    type Output = Uint256;

    fn bitor(self, rhs: Self) -> Self::Output {
        Uint256(self.0 | rhs.0)
    }
}

impl std::ops::BitXor for Uint256 {
    type Output = Uint256;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Uint256(self.0 ^ rhs.0)
    }
}

impl std::ops::Shl<usize> for Uint256 {
    type Output = Uint256;

    fn shl(self, rhs: usize) -> Self::Output {
        Uint256(self.0 << rhs)
    }
}

impl std::ops::Shr<usize> for Uint256 {
    type Output = Uint256;

    fn shr(self, rhs: usize) -> Self::Output {
        Uint256(self.0 >> rhs)
    }
}

impl fmt::Display for Uint256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Uint256 {
    fn default() -> Self {
        Uint256::zero()
    }
}

impl From<u64> for Uint256 {
    fn from(value: u64) -> Self {
        Uint256::from_u64(value)
    }
}

impl From<u32> for Uint256 {
    fn from(value: u32) -> Self {
        Uint256::from_u32(value)
    }
}

impl From<u8> for Uint256 {
    fn from(value: u8) -> Self {
        Uint256::from_u8(value)
    }
}
