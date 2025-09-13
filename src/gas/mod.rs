use crate::types::Uint256;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GasError {
    #[error("Out of gas: required {required}, available {available}")]
    OutOfGas { required: u64, available: u64 },
    #[error("Gas limit exceeded: {limit}")]
    GasLimitExceeded { limit: u64 },
}

/// Gas costs for EVM operations
pub struct GasCosts {
    // Arithmetic operations
    pub add: u64,
    pub mul: u64,
    pub sub: u64,
    pub div: u64,
    pub sdiv: u64,
    pub mod_: u64,
    pub smod: u64,
    pub addmod: u64,
    pub mulmod: u64,
    pub exp: u64,
    pub signextend: u64,

    // Comparison operations
    pub lt: u64,
    pub gt: u64,
    pub slt: u64,
    pub sgt: u64,
    pub eq: u64,
    pub iszero: u64,
    pub and: u64,
    pub or: u64,
    pub xor: u64,
    pub not: u64,
    pub byte: u64,
    pub shl: u64,
    pub shr: u64,
    pub sar: u64,

    // Keccak256 operation
    pub keccak256: u64,
    pub keccak256_word: u64,

    // Environmental information
    pub address: u64,
    pub balance: u64,
    pub origin: u64,
    pub caller: u64,
    pub callvalue: u64,
    pub calldataload: u64,
    pub calldatasize: u64,
    pub calldatacopy: u64,
    pub codesize: u64,
    pub codecopy: u64,
    pub gasprice: u64,
    pub extcodesize: u64,
    pub extcodecopy: u64,
    pub returndatasize: u64,
    pub returndatacopy: u64,
    pub extcodehash: u64,

    // Block information
    pub blockhash: u64,
    pub coinbase: u64,
    pub timestamp: u64,
    pub number: u64,
    pub difficulty: u64,
    pub gaslimit: u64,
    pub chainid: u64,
    pub selfbalance: u64,

    // Storage operations
    pub sload: u64,
    pub sstore: u64,
    pub sstore_set: u64,
    pub sstore_reset: u64,
    pub sstore_clear: u64,

    // Memory operations
    pub mload: u64,
    pub mstore: u64,
    pub mstore8: u64,
    pub msize: u64,

    // Stack operations
    pub push: u64,
    pub dup: u64,
    pub swap: u64,
    pub pop: u64,

    // Control flow
    pub jump: u64,
    pub jumpi: u64,
    pub pc: u64,
    pub jumpdest: u64,

    // Logging operations
    pub log0: u64,
    pub log1: u64,
    pub log2: u64,
    pub log3: u64,
    pub log4: u64,
    pub log_topic: u64,
    pub log_data: u64,

    // System operations
    pub create: u64,
    pub call: u64,
    pub callcode: u64,
    pub delegatecall: u64,
    pub staticcall: u64,
    pub return_: u64,
    pub revert: u64,
    pub selfdestruct: u64,
    pub selfdestruct_refund: u64,

    // Base costs
    pub base: u64,
    pub very_low: u64,
    pub low: u64,
    pub mid: u64,
    pub high: u64,
    pub warm_storage_read: u64,
    pub cold_storage_read: u64,
    pub access_list_storage_key: u64,
    pub access_list_address: u64,
}

impl Default for GasCosts {
    fn default() -> Self {
        GasCosts {
            // Base costs
            base: 2,
            very_low: 3,
            low: 5,
            mid: 8,
            high: 10,

            // Arithmetic operations
            add: 3,
            mul: 5,
            sub: 3,
            div: 5,
            sdiv: 5,
            mod_: 5,
            smod: 5,
            addmod: 8,
            mulmod: 8,
            exp: 10,
            signextend: 5,

            // Comparison operations
            lt: 3,
            gt: 3,
            slt: 3,
            sgt: 3,
            eq: 3,
            iszero: 3,
            and: 3,
            or: 3,
            xor: 3,
            not: 3,
            byte: 3,
            shl: 3,
            shr: 3,
            sar: 3,

            // Keccak256 operation
            keccak256: 30,
            keccak256_word: 6,

            // Environmental information
            address: 2,
            balance: 100,
            origin: 2,
            caller: 2,
            callvalue: 2,
            calldataload: 3,
            calldatasize: 2,
            calldatacopy: 3,
            codesize: 2,
            codecopy: 3,
            gasprice: 2,
            extcodesize: 100,
            extcodecopy: 100,
            returndatasize: 2,
            returndatacopy: 3,
            extcodehash: 100,

            // Block information
            blockhash: 20,
            coinbase: 2,
            timestamp: 2,
            number: 2,
            difficulty: 2,
            gaslimit: 2,
            chainid: 2,
            selfbalance: 5,

            // Storage operations
            sload: 100,
            sstore: 100,
            sstore_set: 20000,
            sstore_reset: 5000,
            sstore_clear: 15000,

            // Memory operations
            mload: 3,
            mstore: 3,
            mstore8: 3,
            msize: 2,

            // Stack operations
            push: 3,
            dup: 3,
            swap: 3,
            pop: 2,

            // Control flow
            jump: 8,
            jumpi: 10,
            pc: 2,
            jumpdest: 1,

            // Logging operations
            log0: 375,
            log1: 750,
            log2: 1125,
            log3: 1500,
            log4: 1875,
            log_topic: 375,
            log_data: 8,

            // System operations
            create: 32000,
            call: 100,
            callcode: 100,
            delegatecall: 100,
            staticcall: 100,
            return_: 0,
            revert: 0,
            selfdestruct: 5000,
            selfdestruct_refund: 24000,

            // Access list costs
            warm_storage_read: 100,
            cold_storage_read: 2100,
            access_list_storage_key: 1900,
            access_list_address: 2400,
        }
    }
}

/// Gas meter for tracking gas consumption
pub struct GasMeter {
    /// Available gas
    available: u64,
    /// Gas limit
    limit: u64,
    /// Gas costs configuration
    costs: GasCosts,
}

impl GasMeter {
    /// Create a new gas meter with the given gas limit
    pub fn new(gas_limit: u64) -> Self {
        GasMeter {
            available: gas_limit,
            limit: gas_limit,
            costs: GasCosts::default(),
        }
    }

    /// Create a new gas meter with custom gas costs
    pub fn with_costs(gas_limit: u64, costs: GasCosts) -> Self {
        GasMeter {
            available: gas_limit,
            limit: gas_limit,
            costs,
        }
    }

    /// Get the available gas
    pub fn available(&self) -> u64 {
        self.available
    }

    /// Get the gas limit
    pub fn limit(&self) -> u64 {
        self.limit
    }

    /// Get the gas used
    pub fn used(&self) -> u64 {
        self.limit - self.available
    }

    /// Consume gas for an operation
    pub fn consume(&mut self, amount: u64) -> Result<(), GasError> {
        if amount > self.available {
            return Err(GasError::OutOfGas {
                required: amount,
                available: self.available,
            });
        }
        self.available -= amount;
        Ok(())
    }

    /// Refund gas (up to half of the gas used)
    pub fn refund(&mut self, amount: u64) {
        let max_refund = self.used() / 2;
        let refund = amount.min(max_refund);
        self.available += refund;
    }

    /// Check if there's enough gas for an operation
    pub fn has_gas(&self, amount: u64) -> bool {
        self.available >= amount
    }

    /// Get the gas costs configuration
    pub fn costs(&self) -> &GasCosts {
        &self.costs
    }

    /// Calculate gas cost for memory expansion
    pub fn memory_expansion_cost(&self, current_size: usize, new_size: usize) -> u64 {
        if new_size <= current_size {
            return 0;
        }
        
        let current_words = (current_size + 31) / 32;
        let new_words = (new_size + 31) / 32;
        
        if new_words <= current_words {
            return 0;
        }
        
        let additional_words = new_words - current_words;
        let cost = additional_words * 3 + (new_words * new_words) / 512 - (current_words * current_words) / 512;
        cost as u64
    }

    /// Calculate gas cost for Keccak256 operation
    pub fn keccak256_cost(&self, data_size: usize) -> u64 {
        let words = (data_size + 31) / 32;
        self.costs.keccak256 + (words as u64 * self.costs.keccak256_word)
    }

    /// Calculate gas cost for SLOAD operation
    pub fn sload_cost(&self, is_warm: bool) -> u64 {
        if is_warm {
            self.costs.warm_storage_read
        } else {
            self.costs.cold_storage_read
        }
    }

    /// Calculate gas cost for SSTORE operation
    pub fn sstore_cost(&self, current_value: &Uint256, new_value: &Uint256, _original_value: &Uint256) -> u64 {
        if current_value == new_value {
            // No change
            if current_value.is_zero() {
                self.costs.sstore_clear
            } else {
                self.costs.sstore_reset
            }
        } else {
            // Value is changing
            if current_value.is_zero() {
                // Setting a zero value to non-zero
                self.costs.sstore_set
            } else if new_value.is_zero() {
                // Setting a non-zero value to zero
                self.costs.sstore_clear
            } else {
                // Changing from one non-zero value to another
                self.costs.sstore_reset
            }
        }
    }

    /// Calculate gas cost for LOG operation
    pub fn log_cost(&self, topic_count: usize, data_size: usize) -> u64 {
        let base_cost = match topic_count {
            0 => self.costs.log0,
            1 => self.costs.log1,
            2 => self.costs.log2,
            3 => self.costs.log3,
            4 => self.costs.log4,
            _ => return 0, // Invalid topic count
        };
        
        base_cost + (data_size as u64 * self.costs.log_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gas_meter_creation() {
        let meter = GasMeter::new(1000);
        assert_eq!(meter.available(), 1000);
        assert_eq!(meter.limit(), 1000);
        assert_eq!(meter.used(), 0);
    }

    #[test]
    fn test_gas_consumption() {
        let mut meter = GasMeter::new(1000);
        
        assert!(meter.consume(500).is_ok());
        assert_eq!(meter.available(), 500);
        assert_eq!(meter.used(), 500);
        
        assert!(meter.consume(300).is_ok());
        assert_eq!(meter.available(), 200);
        assert_eq!(meter.used(), 800);
        
        // Try to consume more than available
        assert!(meter.consume(300).is_err());
    }

    #[test]
    fn test_gas_refund() {
        let mut meter = GasMeter::new(1000);
        
        // Use some gas
        meter.consume(800).unwrap();
        assert_eq!(meter.used(), 800);
        
        // Refund gas (max refund is half of used gas)
        meter.refund(500);
        assert_eq!(meter.available(), 600); // 200 + 400 (half of 800)
    }

    #[test]
    fn test_memory_expansion_cost() {
        let meter = GasMeter::new(1000);
        
        // Cost for expanding from 0 to 32 bytes
        let cost = meter.memory_expansion_cost(0, 32);
        assert_eq!(cost, 3);
        
        // Cost for expanding from 32 to 64 bytes
        let cost = meter.memory_expansion_cost(32, 64);
        assert_eq!(cost, 3);
    }

    #[test]
    fn test_keccak256_cost() {
        let meter = GasMeter::new(1000);
        
        // Cost for 32 bytes (1 word)
        let cost = meter.keccak256_cost(32);
        assert_eq!(cost, 30 + 6); // base + 1 word
        
        // Cost for 64 bytes (2 words)
        let cost = meter.keccak256_cost(64);
        assert_eq!(cost, 30 + 12); // base + 2 words
    }
}
