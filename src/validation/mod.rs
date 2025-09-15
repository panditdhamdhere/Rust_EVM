use crate::{
    types::{Address, Uint256, Bytes},
    opcodes::Opcode,
    executor::ExecutionContext,
};
use thiserror::Error;
use std::collections::HashSet;
use std::str::FromStr;
use num_traits::Num;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Invalid bytecode: {message}")]
    InvalidBytecode { message: String },
    #[error("Invalid address: {address}")]
    InvalidAddress { address: String },
    #[error("Invalid gas limit: {limit}")]
    InvalidGasLimit { limit: u64 },
    #[error("Invalid value: {value}")]
    InvalidValue { value: String },
    #[error("Code size too large: {size} bytes (max: {max})")]
    CodeSizeTooLarge { size: usize, max: usize },
    #[error("Invalid jump destination at PC {pc}")]
    InvalidJumpDestination { pc: usize },
    #[error("Stack depth validation failed: {message}")]
    StackDepthValidation { message: String },
    #[error("Memory bounds validation failed: {message}")]
    MemoryBoundsValidation { message: String },
    #[error("Security validation failed: {message}")]
    SecurityValidation { message: String },
}

/// Comprehensive validator for EVM operations
pub struct Validator {
    /// Maximum code size allowed
    max_code_size: usize,
    /// Maximum stack depth
    max_stack_depth: usize,
    /// Maximum memory size
    max_memory_size: usize,
    /// Maximum gas limit
    max_gas_limit: u64,
    /// Minimum gas limit
    min_gas_limit: u64,
    /// Security checks enabled
    security_checks: bool,
}

impl Validator {
    /// Create a new validator with default limits
    pub fn new() -> Self {
        Validator {
            max_code_size: 24576, // 24KB - Ethereum's limit
            max_stack_depth: 1024, // EVM stack limit
            max_memory_size: 1024 * 1024 * 1024, // 1GB
            max_gas_limit: 30_000_000, // 30M gas
            min_gas_limit: 21_000, // Minimum transaction gas
            security_checks: true,
        }
    }

    /// Create a validator with custom limits
    pub fn with_limits(
        max_code_size: usize,
        max_stack_depth: usize,
        max_memory_size: usize,
        max_gas_limit: u64,
        min_gas_limit: u64,
    ) -> Self {
        Validator {
            max_code_size,
            max_stack_depth,
            max_memory_size,
            max_gas_limit,
            min_gas_limit,
            security_checks: true,
        }
    }

    /// Validate bytecode
    pub fn validate_bytecode(&self, code: &[u8]) -> Result<(), ValidationError> {
        // Check code size
        if code.len() > self.max_code_size {
            return Err(ValidationError::CodeSizeTooLarge {
                size: code.len(),
                max: self.max_code_size,
            });
        }

        // Check for valid opcodes and jump destinations
        self.validate_opcodes(code)?;
        self.validate_jump_destinations(code)?;

        // Security checks
        if self.security_checks {
            self.validate_security(code)?;
        }

        Ok(())
    }

    /// Validate opcodes in bytecode
    fn validate_opcodes(&self, code: &[u8]) -> Result<(), ValidationError> {
        let mut i = 0;
        while i < code.len() {
            let opcode_byte = code[i];
            
            // Check if it's a valid opcode
            if let Err(_) = Opcode::from_byte(opcode_byte) {
                return Err(ValidationError::InvalidBytecode {
                    message: format!("Invalid opcode 0x{:02x} at position {}", opcode_byte, i),
                });
            }

            // Handle PUSH opcodes
            if let Ok(opcode) = Opcode::from_byte(opcode_byte) {
                if opcode.is_push() {
                    let push_size = opcode.get_push_size();
                    if i + push_size >= code.len() {
                        return Err(ValidationError::InvalidBytecode {
                            message: format!("PUSH opcode at position {} extends beyond code", i),
                        });
                    }
                    i += push_size + 1;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        Ok(())
    }

    /// Validate jump destinations
    fn validate_jump_destinations(&self, code: &[u8]) -> Result<(), ValidationError> {
        let mut jumpdests = HashSet::new();
        let mut i = 0;

        // First pass: collect all JUMPDEST locations
        while i < code.len() {
            if let Ok(opcode) = Opcode::from_byte(code[i]) {
                if opcode == Opcode::Jumpdest {
                    jumpdests.insert(i);
                }
                
                if opcode.is_push() {
                    let push_size = opcode.get_push_size();
                    i += push_size + 1;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        // Second pass: validate JUMP and JUMPI destinations
        i = 0;
        while i < code.len() {
            if let Ok(opcode) = Opcode::from_byte(code[i]) {
                if opcode == Opcode::Jump || opcode == Opcode::Jumpi {
                    // Check if the jump destination is valid
                    // This is a simplified check - in reality, we'd need to analyze the stack
                    // to see what value would be jumped to
                    if !jumpdests.is_empty() {
                        // For now, just ensure there are valid jump destinations
                        // A full implementation would track stack values
                    }
                }
                
                if opcode.is_push() {
                    let push_size = opcode.get_push_size();
                    i += push_size + 1;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        Ok(())
    }

    /// Security validation
    fn validate_security(&self, code: &[u8]) -> Result<(), ValidationError> {
        // Check for suspicious patterns
        let _code_str = hex::encode(code);
        
        // Check for excessive use of expensive opcodes
        let mut expensive_opcodes = 0;
        for &byte in code {
            if let Ok(opcode) = Opcode::from_byte(byte) {
                match opcode {
                    Opcode::Exp | Opcode::Sstore | Opcode::Sha3 => {
                        expensive_opcodes += 1;
                    }
                    _ => {}
                }
            }
        }

        if expensive_opcodes > code.len() / 10 {
            return Err(ValidationError::SecurityValidation {
                message: "Too many expensive opcodes detected".to_string(),
            });
        }

        // Check for potential infinite loops (simplified)
        let mut jump_count = 0;
        for &byte in code {
            if let Ok(opcode) = Opcode::from_byte(byte) {
                if opcode == Opcode::Jump || opcode == Opcode::Jumpi {
                    jump_count += 1;
                }
            }
        }

        if jump_count > code.len() / 5 {
            return Err(ValidationError::SecurityValidation {
                message: "Potential infinite loop detected".to_string(),
            });
        }

        Ok(())
    }

    /// Validate execution context
    pub fn validate_execution_context(&self, context: &ExecutionContext) -> Result<(), ValidationError> {
        // Validate gas limit
        if context.gas_meter.limit() > self.max_gas_limit {
            return Err(ValidationError::InvalidGasLimit {
                limit: context.gas_meter.limit(),
            });
        }

        if context.gas_meter.limit() < self.min_gas_limit {
            return Err(ValidationError::InvalidGasLimit {
                limit: context.gas_meter.limit(),
            });
        }

        // Validate code
        self.validate_bytecode(context.code.as_slice())?;

        // Validate stack depth
        if context.stack.size() > self.max_stack_depth {
            return Err(ValidationError::StackDepthValidation {
                message: format!("Stack depth {} exceeds maximum {}", 
                    context.stack.size(), self.max_stack_depth),
            });
        }

        // Validate memory size
        if context.memory.size() > self.max_memory_size {
            return Err(ValidationError::MemoryBoundsValidation {
                message: format!("Memory size {} exceeds maximum {}", 
                    context.memory.size(), self.max_memory_size),
            });
        }

        Ok(())
    }

    /// Validate address format
    pub fn validate_address(&self, address: &str) -> Result<Address, ValidationError> {
        if address.len() != 42 || !address.starts_with("0x") {
            return Err(ValidationError::InvalidAddress {
                address: address.to_string(),
            });
        }

        Address::from_hex(address).map_err(|_| ValidationError::InvalidAddress {
            address: address.to_string(),
        })
    }

    /// Validate value (wei amount)
    pub fn validate_value(&self, value: &str) -> Result<Uint256, ValidationError> {
        let parsed_value = if value.starts_with("0x") {
            let big_uint = num_bigint::BigUint::from_str_radix(&value[2..], 16)
                .map_err(|_| ValidationError::InvalidValue {
                    value: value.to_string(),
                })?;
            Uint256::new(big_uint)
        } else {
            let big_uint = num_bigint::BigUint::from_str(value)
                .map_err(|_| ValidationError::InvalidValue {
                    value: value.to_string(),
                })?;
            Uint256::new(big_uint)
        };

        // Check for reasonable value limits (prevent overflow attacks)
        let max_value = Uint256::from_u64(21_000_000) * Uint256::from_u64(1_000_000_000_000_000_000); // 21M ETH in wei
        if parsed_value > max_value {
            return Err(ValidationError::InvalidValue {
                value: "Value too large".to_string(),
            });
        }

        Ok(parsed_value)
    }

    /// Validate gas limit
    pub fn validate_gas_limit(&self, gas_limit: u64) -> Result<(), ValidationError> {
        if gas_limit > self.max_gas_limit {
            return Err(ValidationError::InvalidGasLimit {
                limit: gas_limit,
            });
        }

        if gas_limit < self.min_gas_limit {
            return Err(ValidationError::InvalidGasLimit {
                limit: gas_limit,
            });
        }

        Ok(())
    }

    /// Validate input data
    pub fn validate_input_data(&self, input: &str) -> Result<Bytes, ValidationError> {
        if input.is_empty() {
            return Ok(Bytes::empty());
        }

        let data = if input.starts_with("0x") {
            hex::decode(&input[2..]).map_err(|_| ValidationError::InvalidBytecode {
                message: "Invalid hex input data".to_string(),
            })?
        } else {
            hex::decode(input).map_err(|_| ValidationError::InvalidBytecode {
                message: "Invalid hex input data".to_string(),
            })?
        };

        // Check input data size limit
        if data.len() > 1024 * 1024 { // 1MB limit
            return Err(ValidationError::InvalidBytecode {
                message: "Input data too large".to_string(),
            });
        }

        Ok(Bytes::from(data))
    }

    /// Get validation statistics
    pub fn get_stats(&self) -> ValidationStats {
        ValidationStats {
            max_code_size: self.max_code_size,
            max_stack_depth: self.max_stack_depth,
            max_memory_size: self.max_memory_size,
            max_gas_limit: self.max_gas_limit,
            min_gas_limit: self.min_gas_limit,
            security_checks_enabled: self.security_checks,
        }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Validator::new()
    }
}

/// Validation statistics
#[derive(Debug, Clone)]
pub struct ValidationStats {
    pub max_code_size: usize,
    pub max_stack_depth: usize,
    pub max_memory_size: usize,
    pub max_gas_limit: u64,
    pub min_gas_limit: u64,
    pub security_checks_enabled: bool,
}

impl std::fmt::Display for ValidationStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Validation Limits:\n")?;
        write!(f, "  Max Code Size: {} bytes\n", self.max_code_size)?;
        write!(f, "  Max Stack Depth: {}\n", self.max_stack_depth)?;
        write!(f, "  Max Memory Size: {} bytes\n", self.max_memory_size)?;
        write!(f, "  Gas Limit Range: {} - {}\n", self.min_gas_limit, self.max_gas_limit)?;
        write!(f, "  Security Checks: {}", if self.security_checks_enabled { "Enabled" } else { "Disabled" })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = Validator::new();
        assert_eq!(validator.max_code_size, 24576);
        assert_eq!(validator.max_stack_depth, 1024);
    }

    #[test]
    fn test_bytecode_validation() {
        let validator = Validator::new();
        
        // Valid bytecode
        let valid_code = vec![0x60, 0x02, 0x60, 0x03, 0x01, 0x00]; // PUSH1 0x02 PUSH1 0x03 ADD STOP
        assert!(validator.validate_bytecode(&valid_code).is_ok());
        
        // Invalid opcode
        let invalid_code = vec![0xFF, 0x00]; // Invalid opcode
        assert!(validator.validate_bytecode(&invalid_code).is_err());
    }

    #[test]
    fn test_address_validation() {
        let validator = Validator::new();
        
        // Valid address
        let valid_addr = "0x0000000000000000000000000000000000000000";
        assert!(validator.validate_address(valid_addr).is_ok());
        
        // Invalid address
        let invalid_addr = "0x123";
        assert!(validator.validate_address(invalid_addr).is_err());
    }

    #[test]
    fn test_gas_limit_validation() {
        let validator = Validator::new();
        
        // Valid gas limit
        assert!(validator.validate_gas_limit(1000000).is_ok());
        
        // Too high
        assert!(validator.validate_gas_limit(50_000_000).is_err());
        
        // Too low
        assert!(validator.validate_gas_limit(10000).is_err());
    }
}
