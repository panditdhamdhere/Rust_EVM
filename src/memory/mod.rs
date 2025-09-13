use crate::types::Uint256;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Memory access out of bounds: offset {offset}, size {size}")]
    OutOfBounds { offset: usize, size: usize },
    #[error("Memory expansion failed: new size {size}")]
    ExpansionFailed { size: usize },
}

/// EVM Memory - expandable byte array
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    /// Create a new empty memory
    pub fn new() -> Self {
        Memory {
            data: Vec::new(),
        }
    }

    /// Get the current memory size in bytes
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Ensure memory is at least the specified size
    pub fn ensure_size(&mut self, size: usize) -> Result<(), MemoryError> {
        if self.data.len() < size {
            self.data.resize(size, 0);
        }
        Ok(())
    }

    /// Read a byte at the given offset
    pub fn read_byte(&self, offset: usize) -> Result<u8, MemoryError> {
        if offset >= self.data.len() {
            return Ok(0); // Uninitialized memory returns 0
        }
        Ok(self.data[offset])
    }

    /// Write a byte at the given offset
    pub fn write_byte(&mut self, offset: usize, value: u8) -> Result<(), MemoryError> {
        self.ensure_size(offset + 1)?;
        self.data[offset] = value;
        Ok(())
    }

    /// Read a 32-byte word at the given offset
    pub fn read_word(&self, offset: usize) -> Result<Uint256, MemoryError> {
        // For reading, we don't need to expand memory, just return zeros for uninitialized areas
        
        let mut bytes = [0u8; 32];
        for i in 0..32 {
            bytes[i] = if offset + i < self.data.len() {
                self.data[offset + i]
            } else {
                0
            };
        }
        
        Ok(Uint256::from_bytes_be(&bytes))
    }

    /// Write a 32-byte word at the given offset
    pub fn write_word(&mut self, offset: usize, value: Uint256) -> Result<(), MemoryError> {
        self.ensure_size(offset + 32)?;
        
        let bytes = value.to_bytes_be();
        for i in 0..32 {
            self.data[offset + i] = bytes[i];
        }
        
        Ok(())
    }

    /// Read a slice of bytes at the given offset
    pub fn read_bytes(&self, offset: usize, size: usize) -> Result<Vec<u8>, MemoryError> {
        if offset + size > self.data.len() {
            // Return zero-padded bytes for uninitialized memory
            let mut result = vec![0u8; size];
            let available = self.data.len().saturating_sub(offset);
            if available > 0 {
                result[..available].copy_from_slice(&self.data[offset..offset + available]);
            }
            return Ok(result);
        }
        
        Ok(self.data[offset..offset + size].to_vec())
    }

    /// Write a slice of bytes at the given offset
    pub fn write_bytes(&mut self, offset: usize, data: &[u8]) -> Result<(), MemoryError> {
        self.ensure_size(offset + data.len())?;
        
        for (i, &byte) in data.iter().enumerate() {
            self.data[offset + i] = byte;
        }
        
        Ok(())
    }

    /// Copy memory from one location to another
    pub fn copy(&mut self, dest_offset: usize, src_offset: usize, size: usize) -> Result<(), MemoryError> {
        if size == 0 {
            return Ok(());
        }
        
        self.ensure_size(dest_offset + size)?;
        self.ensure_size(src_offset + size)?;
        
        // Handle overlapping regions
        if dest_offset < src_offset + size && src_offset < dest_offset + size {
            // Overlapping copy - use temporary buffer
            let temp = self.data[src_offset..src_offset + size].to_vec();
            self.data[dest_offset..dest_offset + size].copy_from_slice(&temp);
        } else {
            // Non-overlapping copy
            self.data.copy_within(src_offset..src_offset + size, dest_offset);
        }
        
        Ok(())
    }

    /// Clear all memory
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get the raw memory data (for debugging)
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Calculate the gas cost for memory expansion
    pub fn expansion_cost(&self, new_size: usize) -> u64 {
        let current_size = self.size();
        if new_size <= current_size {
            return 0;
        }
        
        // Gas cost calculation based on EVM specification
        let current_words = (current_size + 31) / 32;
        let new_words = (new_size + 31) / 32;
        
        if new_words <= current_words {
            return 0;
        }
        
        let additional_words = new_words - current_words;
        let cost = additional_words * 3 + (new_words * new_words) / 512 - (current_words * current_words) / 512;
        cost as u64
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_read_write_byte() {
        let mut memory = Memory::new();
        
        assert_eq!(memory.read_byte(0).unwrap(), 0);
        assert!(memory.write_byte(0, 42).is_ok());
        assert_eq!(memory.read_byte(0).unwrap(), 42);
    }

    #[test]
    fn test_memory_read_write_word() {
        let mut memory = Memory::new();
        let value = Uint256::from_u32(0x12345678);
        
        assert!(memory.write_word(0, value.clone()).is_ok());
        assert_eq!(memory.read_word(0).unwrap(), value);
    }

    #[test]
    fn test_memory_uninitialized_read() {
        let memory = Memory::new();
        
        // Reading from uninitialized memory should return 0
        assert_eq!(memory.read_byte(100).unwrap(), 0);
        assert_eq!(memory.read_word(100).unwrap(), Uint256::zero());
    }

    #[test]
    fn test_memory_copy() {
        let mut memory = Memory::new();
        
        // Write some data
        let data = b"Hello, World!";
        memory.write_bytes(0, data).unwrap();
        
        // Copy to another location
        memory.copy(20, 0, data.len()).unwrap();
        
        // Verify the copy
        let copied = memory.read_bytes(20, data.len()).unwrap();
        assert_eq!(copied, data);
    }

    #[test]
    fn test_memory_expansion_cost() {
        let memory = Memory::new();
        
        // Cost for expanding to 32 bytes (1 word)
        let cost = memory.expansion_cost(32);
        assert_eq!(cost, 3); // 1 word * 3 + 1*1/512 - 0*0/512 = 3
        
        // Cost for expanding to 64 bytes (2 words)
        let cost = memory.expansion_cost(64);
        assert_eq!(cost, 6); // 2 words * 3 + 2*2/512 - 0*0/512 = 6
    }
}
