use crate::types::{Address, Uint256};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Block context containing blockchain information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockContext {
    /// Block number
    pub number: Uint256,
    /// Block timestamp
    pub timestamp: Uint256,
    /// Block difficulty
    pub difficulty: Uint256,
    /// Gas limit for the block
    pub gas_limit: Uint256,
    /// Block coinbase (miner address)
    pub coinbase: Address,
    /// Chain ID
    pub chain_id: Uint256,
    /// Block hash (for the previous block)
    pub block_hash: Uint256,
    /// Base fee (for EIP-1559)
    pub base_fee: Uint256,
}

impl BlockContext {
    /// Create a new block context with default values
    pub fn new() -> Self {
        BlockContext {
            number: Uint256::from_u32(1),
            timestamp: Uint256::from_u32(1640995200), // Jan 1, 2022
            difficulty: Uint256::from_u32(0x200000), // 2^21
            gas_limit: Uint256::from_u32(30000000), // 30M gas
            coinbase: Address::zero(),
            chain_id: Uint256::from_u32(1), // Mainnet
            block_hash: Uint256::zero(),
            base_fee: Uint256::from_u64(20_000_000_000), // 20 gwei
        }
    }

    /// Create a block context with custom values
    pub fn with_values(
        number: Uint256,
        timestamp: Uint256,
        difficulty: Uint256,
        gas_limit: Uint256,
        coinbase: Address,
        chain_id: Uint256,
        block_hash: Uint256,
        base_fee: Uint256,
    ) -> Self {
        BlockContext {
            number,
            timestamp,
            difficulty,
            gas_limit,
            coinbase,
            chain_id,
            block_hash,
            base_fee,
        }
    }

    /// Get block hash for a given block number
    /// In a real implementation, this would query the blockchain
    pub fn get_block_hash(&self, block_number: &Uint256) -> Uint256 {
        // For demo purposes, return a deterministic hash
        if block_number == &self.number {
            self.block_hash.clone()
        } else {
            // Generate a deterministic hash based on block number
            let mut hash_bytes = [0u8; 32];
            let block_bytes = block_number.to_bytes_be();
            for (i, &byte) in block_bytes.iter().enumerate() {
                if i < 32 {
                    hash_bytes[i] = byte;
                }
            }
            Uint256::from_bytes_be(&hash_bytes)
        }
    }
}

impl Default for BlockContext {
    fn default() -> Self {
        BlockContext::new()
    }
}

impl fmt::Display for BlockContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block #{} (timestamp: {}, gas_limit: {})", 
            self.number, self.timestamp, self.gas_limit)
    }
}

/// Transaction context containing transaction-specific information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionContext {
    /// Gas price
    pub gas_price: Uint256,
    /// Origin (original sender)
    pub origin: Address,
    /// Gas limit for this transaction
    pub gas_limit: Uint256,
    /// Transaction hash
    pub tx_hash: Uint256,
    /// Transaction nonce
    pub nonce: Uint256,
}

impl TransactionContext {
    /// Create a new transaction context
    pub fn new() -> Self {
        TransactionContext {
            gas_price: Uint256::from_u64(20_000_000_000), // 20 gwei
            origin: Address::zero(),
            gas_limit: Uint256::from_u32(1000000), // 1M gas
            tx_hash: Uint256::zero(),
            nonce: Uint256::zero(),
        }
    }

    /// Create a transaction context with custom values
    pub fn with_values(
        gas_price: Uint256,
        origin: Address,
        gas_limit: Uint256,
        tx_hash: Uint256,
        nonce: Uint256,
    ) -> Self {
        TransactionContext {
            gas_price,
            origin,
            gas_limit,
            tx_hash,
            nonce,
        }
    }
}

impl Default for TransactionContext {
    fn default() -> Self {
        TransactionContext::new()
    }
}

impl fmt::Display for TransactionContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tx (gas_price: {}, origin: {})", 
            self.gas_price, self.origin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_context_creation() {
        let block = BlockContext::new();
        assert_eq!(block.number, Uint256::from_u32(1));
        assert_eq!(block.chain_id, Uint256::from_u32(1));
    }

    #[test]
    fn test_transaction_context_creation() {
        let tx = TransactionContext::new();
        assert_eq!(tx.gas_price, Uint256::from_u32(20_000_000_000));
        assert_eq!(tx.origin, Address::zero());
    }

    #[test]
    fn test_block_hash_generation() {
        let block = BlockContext::new();
        let hash = block.get_block_hash(&Uint256::from_u32(1));
        assert_eq!(hash, block.block_hash);
    }
}
