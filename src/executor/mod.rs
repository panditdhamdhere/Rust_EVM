use crate::{
    types::{Address, Uint256, Bytes, Hash},
    stack::{Stack, StackError},
    memory::{Memory, MemoryError},
    storage::{Storage, StorageError},
    gas::{GasMeter, GasError},
    opcodes::{Opcode, OpcodeError},
    events::{EventLogger, EventLog},
    block::{BlockContext, TransactionContext},
};
use thiserror::Error;
use sha3::{Digest, Keccak256};

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Stack error: {0}")]
    Stack(#[from] StackError),
    #[error("Memory error: {0}")]
    Memory(#[from] MemoryError),
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("Gas error: {0}")]
    Gas(#[from] GasError),
    #[error("Opcode error: {0}")]
    Opcode(#[from] OpcodeError),
    #[error("Invalid instruction: {0}")]
    InvalidInstruction(String),
    #[error("Execution halted: {reason}")]
    Halted { reason: String },
}

/// Execution context for EVM
pub struct ExecutionContext {
    /// Program counter
    pub pc: usize,
    /// Stack
    pub stack: Stack,
    /// Memory
    pub memory: Memory,
    /// Storage
    pub storage: Storage,
    /// Gas meter
    pub gas_meter: GasMeter,
    /// Event logger
    pub event_logger: EventLogger,
    /// Block context
    pub block_context: BlockContext,
    /// Transaction context
    pub transaction_context: TransactionContext,
    /// Current account address
    pub address: Address,
    /// Caller address
    pub caller: Address,
    /// Call value
    pub call_value: Uint256,
    /// Input data
    pub input_data: Bytes,
    /// Code to execute
    pub code: Bytes,
    /// Return data
    pub return_data: Bytes,
    /// Whether execution should continue
    pub should_continue: bool,
    /// Whether execution was successful
    pub success: bool,
}

impl ExecutionContext {
    /// Create a new execution context
    pub fn new(
        address: Address,
        caller: Address,
        call_value: Uint256,
        input_data: Bytes,
        code: Bytes,
        gas_limit: u64,
    ) -> Self {
        ExecutionContext {
            pc: 0,
            stack: Stack::new(),
            memory: Memory::new(),
            storage: Storage::new(),
            gas_meter: GasMeter::new(gas_limit),
            event_logger: EventLogger::new(),
            block_context: BlockContext::new(),
            transaction_context: TransactionContext::new(),
            address,
            caller,
            call_value,
            input_data,
            code,
            return_data: Bytes::empty(),
            should_continue: true,
            success: false,
        }
    }

    /// Get the current instruction
    pub fn current_instruction(&self) -> Result<u8, ExecutionError> {
        if self.pc >= self.code.len() {
            return Err(ExecutionError::InvalidInstruction("Program counter out of bounds".to_string()));
        }
        Ok(self.code.as_slice()[self.pc])
    }

    /// Advance the program counter
    pub fn advance_pc(&mut self, steps: usize) {
        self.pc += steps;
    }

    /// Set the program counter (for jumps)
    pub fn set_pc(&mut self, pc: usize) -> Result<(), ExecutionError> {
        if pc >= self.code.len() {
            return Err(ExecutionError::InvalidInstruction("Jump destination out of bounds".to_string()));
        }
        self.pc = pc;
        Ok(())
    }

    /// Halt execution
    pub fn halt(&mut self, success: bool, reason: String) {
        self.should_continue = false;
        self.success = success;
        if !success {
            log::error!("Execution halted: {}", reason);
        }
    }
}

/// EVM Executor
pub struct Executor {
    /// Execution context
    context: ExecutionContext,
}

impl Executor {
    /// Create a new executor
    pub fn new(context: ExecutionContext) -> Self {
        Executor { context }
    }

    /// Execute the EVM code
    pub fn execute(&mut self) -> Result<ExecutionResult, ExecutionError> {
        while self.context.should_continue && self.context.pc < self.context.code.len() {
            self.step()?;
        }

        // If we reached the end of code without explicit halt, consider it successful
        if self.context.should_continue && self.context.pc >= self.context.code.len() {
            self.context.success = true;
        }

        Ok(ExecutionResult {
            success: self.context.success,
            return_data: self.context.return_data.clone(),
            gas_used: self.context.gas_meter.used(),
            gas_remaining: self.context.gas_meter.available(),
            logs: self.context.event_logger.logs().to_vec(),
        })
    }

    /// Execute a single step
    pub fn step(&mut self) -> Result<(), ExecutionError> {
        // Check if we're out of bounds
        if self.context.pc >= self.context.code.len() {
            return Err(ExecutionError::InvalidInstruction("Program counter out of bounds".to_string()));
        }

        let opcode_byte = self.context.current_instruction()?;
        let opcode = Opcode::from_byte(opcode_byte)?;

        // Validate stack requirements
        self.validate_stack_requirements(&opcode)?;

        // Handle push opcodes specially
        if opcode.is_push() {
            self.execute_push(opcode)?;
        } else {
            self.execute_opcode(opcode)?;
        }

        Ok(())
    }

    /// Validate stack requirements for an opcode
    fn validate_stack_requirements(&self, opcode: &Opcode) -> Result<(), ExecutionError> {
        let required_items = opcode.pop_count();
        if self.context.stack.size() < required_items {
            return Err(ExecutionError::Stack(StackError::Underflow));
        }
        
        let max_stack_size = self.context.stack.max_size();
        if opcode.is_push() && self.context.stack.size() >= max_stack_size {
            return Err(ExecutionError::Stack(StackError::Overflow));
        }
        
        Ok(())
    }

    /// Calculate gas cost for an opcode
    fn calculate_gas_cost(&self, opcode: &Opcode) -> Result<u64, ExecutionError> {
        let costs = self.context.gas_meter.costs();
        
        match opcode {
            // Arithmetic operations
            Opcode::Add => Ok(costs.add),
            Opcode::Mul => Ok(costs.mul),
            Opcode::Sub => Ok(costs.sub),
            Opcode::Div => Ok(costs.div),
            Opcode::Mod => Ok(costs.mod_),
            Opcode::Sdiv => Ok(costs.sdiv),
            Opcode::Smod => Ok(costs.smod),
            Opcode::Addmod => Ok(costs.addmod),
            Opcode::Mulmod => Ok(costs.mulmod),
            Opcode::Signextend => Ok(costs.signextend),
            Opcode::Exp => {
                // EXP gas cost is dynamic based on exponent
                if let Ok(exponent) = self.context.stack.peek_at(0) {
                    let exp_bits = exponent.as_biguint().bits();
                    Ok(costs.exp + (exp_bits * 10) as u64)
                } else {
                    Ok(costs.exp)
                }
            },
            
            // Comparison operations
            Opcode::Lt => Ok(costs.lt),
            Opcode::Gt => Ok(costs.gt),
            Opcode::Slt => Ok(costs.slt),
            Opcode::Sgt => Ok(costs.sgt),
            Opcode::Eq => Ok(costs.eq),
            Opcode::Iszero => Ok(costs.iszero),
            
            // Bitwise operations
            Opcode::And => Ok(costs.and),
            Opcode::Or => Ok(costs.or),
            Opcode::Xor => Ok(costs.xor),
            Opcode::Not => Ok(costs.not),
            Opcode::Byte => Ok(costs.byte),
            Opcode::Shl => Ok(costs.shl),
            Opcode::Shr => Ok(costs.shr),
            
            // SHA3 operation
            Opcode::Sha3 => {
                // Dynamic gas cost based on data size
                if let Ok(size) = self.context.stack.peek_at(0) {
                    let size_usize = size.to_u64() as usize;
                    let words = (size_usize + 31) / 32;
                    Ok(costs.keccak256 + (words as u64 * costs.keccak256_word))
                } else {
                    Ok(costs.keccak256)
                }
            },
            
            // Stack operations
            Opcode::Pop => Ok(costs.pop),
            _ if opcode.is_dup() => Ok(costs.dup),
            _ if opcode.is_swap() => Ok(costs.swap),
            
            // Memory operations
            Opcode::Mload => Ok(costs.mload),
            Opcode::Mstore => Ok(costs.mstore),
            Opcode::Mstore8 => Ok(costs.mstore8),
            Opcode::Msize => Ok(costs.msize),
            
            // Storage operations
            Opcode::Sload => Ok(costs.sload),
            Opcode::Sstore => {
                // Dynamic gas cost for SSTORE
                if let (Ok(key), Ok(value)) = (self.context.stack.peek_at(0), self.context.stack.peek_at(1)) {
                    let current_value = self.context.storage.get_storage(&self.context.address, key);
                    if current_value == *value {
                        // No change
                        if current_value.is_zero() {
                            Ok(costs.sstore_clear)
                        } else {
                            Ok(costs.sstore_reset)
                        }
                    } else {
                        // Value is changing
                        if current_value.is_zero() {
                            Ok(costs.sstore_set)
                        } else if value.is_zero() {
                            Ok(costs.sstore_clear)
                        } else {
                            Ok(costs.sstore_reset)
                        }
                    }
                } else {
                    Ok(costs.sstore)
                }
            },
            
            // Environmental information
            Opcode::Address => Ok(costs.address),
            Opcode::Caller => Ok(costs.caller),
            Opcode::Callvalue => Ok(costs.callvalue),
            Opcode::Calldatasize => Ok(costs.calldatasize),
            Opcode::Calldataload => Ok(costs.calldataload),
            Opcode::Codesize => Ok(costs.codesize),
            Opcode::Codecopy => Ok(costs.codecopy),
            Opcode::Balance => Ok(costs.balance),
            
            // Block information
            Opcode::Blockhash => Ok(costs.blockhash),
            Opcode::Coinbase => Ok(costs.coinbase),
            Opcode::Timestamp => Ok(costs.timestamp),
            Opcode::Number => Ok(costs.number),
            Opcode::Difficulty => Ok(costs.difficulty),
            Opcode::Gaslimit => Ok(costs.gaslimit),
            Opcode::Chainid => Ok(costs.chainid),
            Opcode::Selfbalance => Ok(costs.selfbalance),
            
            // Transaction information
            Opcode::Gasprice => Ok(costs.gasprice),
            Opcode::Origin => Ok(costs.origin),
            
            // Control flow
            Opcode::Jump => Ok(costs.jump),
            Opcode::Jumpi => Ok(costs.jumpi),
            Opcode::Pc => Ok(costs.pc),
            Opcode::Jumpdest => Ok(costs.jumpdest),
            
            // Logging operations
            Opcode::Log0 => Ok(costs.log0),
            Opcode::Log1 => Ok(costs.log1),
            Opcode::Log2 => Ok(costs.log2),
            Opcode::Log3 => Ok(costs.log3),
            Opcode::Log4 => Ok(costs.log4),
            
            // System operations
            Opcode::Return => Ok(costs.return_),
            Opcode::Revert => Ok(costs.revert),
            
            _ => Ok(costs.base), // Default base cost for unimplemented opcodes
        }
    }

    /// Execute a push opcode
    fn execute_push(&mut self, opcode: Opcode) -> Result<(), ExecutionError> {
        // Consume gas for push operation
        let gas_cost = self.context.gas_meter.costs().push;
        self.context.gas_meter.consume(gas_cost)?;
        
        let push_size = opcode.get_push_size();
        self.context.advance_pc(1);

        if self.context.pc + push_size > self.context.code.len() {
            return Err(ExecutionError::InvalidInstruction("Push data extends beyond code".to_string()));
        }

        let mut value_bytes = [0u8; 32];
        let start = self.context.pc;
        let end = start + push_size;
        value_bytes[32 - push_size..].copy_from_slice(&self.context.code.as_slice()[start..end]);

        let value = Uint256::from_bytes_be(&value_bytes);
        self.context.stack.push(value)?;
        self.context.advance_pc(push_size);

        Ok(())
    }

    /// Execute an opcode
    fn execute_opcode(&mut self, opcode: Opcode) -> Result<(), ExecutionError> {
        // Calculate and consume gas for this opcode
        let gas_cost = self.calculate_gas_cost(&opcode)?;
        self.context.gas_meter.consume(gas_cost)?;
        
        self.context.advance_pc(1);

        match opcode {
            // Stop and arithmetic operations
            Opcode::Stop => {
                self.context.halt(true, "STOP instruction".to_string());
            }
            Opcode::Add => {
                let a = self.context.stack.pop()?;
                let b = self.context.stack.pop()?;
                let result = a + b;
                self.context.stack.push(result)?;
            }
            Opcode::Mul => {
                let a = self.context.stack.pop()?;
                let b = self.context.stack.pop()?;
                let result = a * b;
                self.context.stack.push(result)?;
            }
            Opcode::Sub => {
                let a = self.context.stack.pop()?;
                let b = self.context.stack.pop()?;
                let result = a - b;
                self.context.stack.push(result)?;
            }
            Opcode::Div => {
                let a = self.context.stack.pop()?;
                let b = self.context.stack.pop()?;
                let result = if b.is_zero() { Uint256::zero() } else { a / b };
                self.context.stack.push(result)?;
            }
            Opcode::Mod => {
                let a = self.context.stack.pop()?;
                let b = self.context.stack.pop()?;
                let result = if b.is_zero() { Uint256::zero() } else { a % b };
                self.context.stack.push(result)?;
            }
            Opcode::Exp => {
                let base = self.context.stack.pop()?;
                let exponent = self.context.stack.pop()?;
                // For simplicity, we'll use a basic implementation
                // In a real EVM, this would be more complex
                let result = if exponent.is_zero() {
                    Uint256::one()
                } else if base.is_zero() {
                    Uint256::zero()
                } else {
                    // This is a simplified implementation
                    // A real EVM would handle large exponents more efficiently
                    let mut result = Uint256::one();
                    let mut exp = exponent;
                    let mut base_val = base;
                    while !exp.is_zero() {
                        if exp.as_biguint() % 2u32 == 1u32.into() {
                            result = result * base_val.clone();
                        }
                        base_val = base_val.clone() * base_val.clone();
                        exp = exp / Uint256::from_u32(2);
                    }
                    result
                };
                self.context.stack.push(result)?;
            }
            Opcode::Sdiv => {
                let a = self.context.stack.pop()?;
                let b = self.context.stack.pop()?;
                if b.is_zero() {
                    self.context.stack.push(Uint256::zero())?;
                } else {
                    // Signed division: convert to signed, divide, convert back
                    let a_signed = self.uint256_to_signed(&a);
                    let b_signed = self.uint256_to_signed(&b);
                    let result_signed = a_signed / b_signed;
                    let result = self.signed_to_uint256(result_signed);
                    self.context.stack.push(result)?;
                }
            }
            Opcode::Smod => {
                let a = self.context.stack.pop()?;
                let b = self.context.stack.pop()?;
                if b.is_zero() {
                    self.context.stack.push(Uint256::zero())?;
                } else {
                    // Signed modulo: convert to signed, modulo, convert back
                    let a_signed = self.uint256_to_signed(&a);
                    let b_signed = self.uint256_to_signed(&b);
                    let result_signed = a_signed % b_signed;
                    let result = self.signed_to_uint256(result_signed);
                    self.context.stack.push(result)?;
                }
            }
            Opcode::Addmod => {
                let a = self.context.stack.pop()?;
                let b = self.context.stack.pop()?;
                let m = self.context.stack.pop()?;
                if m.is_zero() {
                    self.context.stack.push(Uint256::zero())?;
                } else {
                    // (a + b) mod m
                    let sum = a.as_biguint() + b.as_biguint();
                    let result = sum % m.as_biguint();
                    self.context.stack.push(Uint256::new(result))?;
                }
            }
            Opcode::Mulmod => {
                let a = self.context.stack.pop()?;
                let b = self.context.stack.pop()?;
                let m = self.context.stack.pop()?;
                if m.is_zero() {
                    self.context.stack.push(Uint256::zero())?;
                } else {
                    // (a * b) mod m
                    let product = a.as_biguint() * b.as_biguint();
                    let result = product % m.as_biguint();
                    self.context.stack.push(Uint256::new(result))?;
                }
            }
            Opcode::Signextend => {
                let b = self.context.stack.pop()?;
                let x = self.context.stack.pop()?;
                
                if b >= Uint256::from_u32(31) {
                    // If b >= 31, return x unchanged
                    self.context.stack.push(x)?;
                } else {
                    let b_usize = b.to_u32() as usize;
                    let x_bytes = x.to_bytes_be();
                    let sign_bit = (x_bytes[31 - b_usize] & 0x80) != 0;
                    
                    let mut result_bytes = [0u8; 32];
                    if sign_bit {
                        // Sign extend with 1s
                        for i in 0..(31 - b_usize) {
                            result_bytes[i] = 0xFF;
                        }
                    }
                    // Copy the significant bytes
                    for i in (31 - b_usize)..32 {
                        result_bytes[i] = x_bytes[i];
                    }
                    
                    self.context.stack.push(Uint256::from_bytes_be(&result_bytes))?;
                }
            }

            // Comparison operations
            Opcode::Lt => {
                let a = self.context.stack.pop()?;
                let b = self.context.stack.pop()?;
                let result = if a < b { Uint256::one() } else { Uint256::zero() };
                self.context.stack.push(result)?;
            }
            Opcode::Gt => {
                let a = self.context.stack.pop()?;
                let b = self.context.stack.pop()?;
                let result = if a > b { Uint256::one() } else { Uint256::zero() };
                self.context.stack.push(result)?;
            }
            Opcode::Slt => {
                // Signed less than (simplified implementation)
                let a = self.context.stack.pop()?;
                let b = self.context.stack.pop()?;
                let result = if a < b { Uint256::one() } else { Uint256::zero() };
                self.context.stack.push(result)?;
            }
            Opcode::Sgt => {
                // Signed greater than (simplified implementation)
                let a = self.context.stack.pop()?;
                let b = self.context.stack.pop()?;
                let result = if a > b { Uint256::one() } else { Uint256::zero() };
                self.context.stack.push(result)?;
            }
            Opcode::Eq => {
                let a = self.context.stack.pop()?;
                let b = self.context.stack.pop()?;
                let result = if a == b { Uint256::one() } else { Uint256::zero() };
                self.context.stack.push(result)?;
            }
            Opcode::Iszero => {
                let a = self.context.stack.pop()?;
                let result = if a.is_zero() { Uint256::one() } else { Uint256::zero() };
                self.context.stack.push(result)?;
            }

            // Bitwise operations
            Opcode::And => {
                let a = self.context.stack.pop()?;
                let b = self.context.stack.pop()?;
                let result = a & b;
                self.context.stack.push(result)?;
            }
            Opcode::Or => {
                let a = self.context.stack.pop()?;
                let b = self.context.stack.pop()?;
                let result = a | b;
                self.context.stack.push(result)?;
            }
            Opcode::Xor => {
                let a = self.context.stack.pop()?;
                let b = self.context.stack.pop()?;
                let result = a ^ b;
                self.context.stack.push(result)?;
            }
            Opcode::Byte => {
                let i = self.context.stack.pop()?;
                let x = self.context.stack.pop()?;
                let result = if i >= Uint256::from_u32(32) {
                    Uint256::zero()
                } else {
                    let byte_index = i.to_u32() as usize;
                    let bytes = x.to_bytes_be();
                    Uint256::from_u8(bytes[byte_index])
                };
                self.context.stack.push(result)?;
            }
            Opcode::Shl => {
                let shift = self.context.stack.pop()?;
                let value = self.context.stack.pop()?;
                let result = if shift >= Uint256::from_u32(256) {
                    Uint256::zero()
                } else {
                    value << (shift.to_u32() as usize)
                };
                self.context.stack.push(result)?;
            }
            Opcode::Shr => {
                let shift = self.context.stack.pop()?;
                let value = self.context.stack.pop()?;
                let result = if shift >= Uint256::from_u32(256) {
                    Uint256::zero()
                } else {
                    value >> (shift.to_u32() as usize)
                };
                self.context.stack.push(result)?;
            }
            Opcode::Not => {
                let a = self.context.stack.pop()?;
                // NOT operation on 256-bit value (bitwise complement)
                let max_uint256 = Uint256::from_bytes_be(&[0xFF; 32]);
                let result = a ^ max_uint256;
                self.context.stack.push(result)?;
            }

            // SHA3 operation
            Opcode::Sha3 => {
                let offset = self.context.stack.pop()?;
                let size = self.context.stack.pop()?;
                let offset_usize = offset.to_u64() as usize;
                let size_usize = size.to_u64() as usize;
                
                // Calculate memory expansion cost
                let new_size = offset_usize + size_usize;
                let expansion_cost = self.context.gas_meter.memory_expansion_cost(
                    self.context.memory.size(), 
                    new_size
                );
                self.context.gas_meter.consume(expansion_cost)?;
                
                // Read data from memory
                let data = self.context.memory.read_bytes(offset_usize, size_usize)?;
                
                // Calculate Keccak256 hash
                let hash = Keccak256::digest(&data);
                let result = Uint256::from_bytes_be(&hash);
                self.context.stack.push(result)?;
            }

            // Stack operations
            Opcode::Pop => {
                self.context.stack.pop()?;
            }
            _ if opcode.is_dup() => {
                let depth = opcode.dup_depth();
                self.context.stack.dup(depth)?;
            }
            _ if opcode.is_swap() => {
                let depth = opcode.swap_depth();
                self.context.stack.swap(depth)?;
            }

            // Memory operations
            Opcode::Mload => {
                let offset = self.context.stack.pop()?;
                let offset_usize = offset.to_u64() as usize;
                
                // Calculate memory expansion cost
                let new_size = offset_usize + 32;
                let expansion_cost = self.context.gas_meter.memory_expansion_cost(
                    self.context.memory.size(), 
                    new_size
                );
                self.context.gas_meter.consume(expansion_cost)?;
                
                let value = self.context.memory.read_word(offset_usize)?;
                self.context.stack.push(value)?;
            }
            Opcode::Mstore => {
                let offset = self.context.stack.pop()?;
                let value = self.context.stack.pop()?;
                let offset_usize = offset.to_u64() as usize;
                
                // Calculate memory expansion cost
                let new_size = offset_usize + 32;
                let expansion_cost = self.context.gas_meter.memory_expansion_cost(
                    self.context.memory.size(), 
                    new_size
                );
                self.context.gas_meter.consume(expansion_cost)?;
                
                self.context.memory.write_word(offset_usize, value)?;
            }
            Opcode::Mstore8 => {
                let offset = self.context.stack.pop()?;
                let value = self.context.stack.pop()?;
                let offset_usize = offset.to_u64() as usize;
                
                // Calculate memory expansion cost
                let new_size = offset_usize + 1;
                let expansion_cost = self.context.gas_meter.memory_expansion_cost(
                    self.context.memory.size(), 
                    new_size
                );
                self.context.gas_meter.consume(expansion_cost)?;
                
                let byte_value = value.to_u8();
                self.context.memory.write_byte(offset_usize, byte_value)?;
            }
            Opcode::Msize => {
                let size = Uint256::from_u32(self.context.memory.size() as u32);
                self.context.stack.push(size)?;
            }

            // Storage operations
            Opcode::Sload => {
                let key = self.context.stack.pop()?;
                let value = self.context.storage.get_storage(&self.context.address, &key);
                self.context.stack.push(value)?;
            }
            Opcode::Sstore => {
                let key = self.context.stack.pop()?;
                let value = self.context.stack.pop()?;
                self.context.storage.set_storage(self.context.address, key, value);
            }

            // Environmental information
            Opcode::Address => {
                self.context.stack.push(Uint256::from_bytes_be(self.context.address.as_bytes()))?;
            }
            Opcode::Caller => {
                self.context.stack.push(Uint256::from_bytes_be(self.context.caller.as_bytes()))?;
            }
            Opcode::Callvalue => {
                self.context.stack.push(self.context.call_value.clone())?;
            }
            Opcode::Calldatasize => {
                let size = Uint256::from_u32(self.context.input_data.len() as u32);
                self.context.stack.push(size)?;
            }
            Opcode::Calldataload => {
                let offset = self.context.stack.pop()?;
                let offset_usize = offset.to_u64() as usize;
                if offset_usize >= self.context.input_data.len() {
                    self.context.stack.push(Uint256::zero())?;
                } else {
                    let mut bytes = [0u8; 32];
                    let remaining = self.context.input_data.len() - offset_usize;
                    let copy_len = remaining.min(32);
                    bytes[..copy_len].copy_from_slice(&self.context.input_data.as_slice()[offset_usize..offset_usize + copy_len]);
                    let value = Uint256::from_bytes_be(&bytes);
                    self.context.stack.push(value)?;
                }
            }
            Opcode::Codesize => {
                let size = Uint256::from_u32(self.context.code.len() as u32);
                self.context.stack.push(size)?;
            }
            Opcode::Codecopy => {
                let dest_offset = self.context.stack.pop()?;
                let offset = self.context.stack.pop()?;
                let size = self.context.stack.pop()?;
                let dest_offset_usize = dest_offset.to_u64() as usize;
                let offset_usize = offset.to_u64() as usize;
                let size_usize = size.to_u64() as usize;
                
                // Calculate memory expansion cost
                let new_size = dest_offset_usize + size_usize;
                let expansion_cost = self.context.gas_meter.memory_expansion_cost(
                    self.context.memory.size(), 
                    new_size
                );
                self.context.gas_meter.consume(expansion_cost)?;
                
                if offset_usize < self.context.code.len() {
                    let end = (offset_usize + size_usize).min(self.context.code.len());
                    let data = &self.context.code.as_slice()[offset_usize..end];
                    self.context.memory.write_bytes(dest_offset_usize, data)?;
                }
            }
            Opcode::Balance => {
                let address_bytes = self.context.stack.pop()?;
                let address_bytes_array = address_bytes.to_bytes_be();
                let address_slice = &address_bytes_array[12..]; // Take last 20 bytes
                let mut address_array = [0u8; 20];
                address_array.copy_from_slice(address_slice);
                let address = Address::new(address_array);
                let balance = self.context.storage.get_balance(&address);
                self.context.stack.push(balance)?;
            }

            // Block information opcodes
            Opcode::Blockhash => {
                let block_number = self.context.stack.pop()?;
                let block_hash = self.context.block_context.get_block_hash(&block_number);
                self.context.stack.push(block_hash)?;
            }
            Opcode::Coinbase => {
                let coinbase_bytes = *self.context.block_context.coinbase.as_bytes();
                let coinbase_uint = Uint256::from_bytes_be(&coinbase_bytes);
                self.context.stack.push(coinbase_uint)?;
            }
            Opcode::Timestamp => {
                self.context.stack.push(self.context.block_context.timestamp.clone())?;
            }
            Opcode::Number => {
                self.context.stack.push(self.context.block_context.number.clone())?;
            }
            Opcode::Difficulty => {
                self.context.stack.push(self.context.block_context.difficulty.clone())?;
            }
            Opcode::Gaslimit => {
                self.context.stack.push(self.context.block_context.gas_limit.clone())?;
            }
            Opcode::Chainid => {
                self.context.stack.push(self.context.block_context.chain_id.clone())?;
            }
            Opcode::Selfbalance => {
                let balance = self.context.storage.get_balance(&self.context.address);
                self.context.stack.push(balance)?;
            }

            // Transaction information opcodes
            Opcode::Gasprice => {
                self.context.stack.push(self.context.transaction_context.gas_price.clone())?;
            }
            Opcode::Origin => {
                let origin_bytes = *self.context.transaction_context.origin.as_bytes();
                let origin_uint = Uint256::from_bytes_be(&origin_bytes);
                self.context.stack.push(origin_uint)?;
            }

            // Control flow
            Opcode::Jump => {
                let dest = self.context.stack.pop()?;
                let dest_usize = dest.to_u64() as usize;
                self.context.set_pc(dest_usize)?;
            }
            Opcode::Jumpi => {
                let dest = self.context.stack.pop()?;
                let condition = self.context.stack.pop()?;
                if !condition.is_zero() {
                    let dest_usize = dest.to_u64() as usize;
                    self.context.set_pc(dest_usize)?;
                }
            }
            Opcode::Pc => {
                let pc = Uint256::from_u32(self.context.pc as u32);
                self.context.stack.push(pc)?;
            }
            Opcode::Jumpdest => {
                // No operation, just a valid jump destination
            }

            // Logging operations
            Opcode::Log0 => {
                let offset = self.context.stack.pop()?;
                let size = self.context.stack.pop()?;
                let offset_usize = offset.to_u64() as usize;
                let size_usize = size.to_u64() as usize;
                
                // Calculate memory expansion cost
                let new_size = offset_usize + size_usize;
                let expansion_cost = self.context.gas_meter.memory_expansion_cost(
                    self.context.memory.size(), 
                    new_size
                );
                self.context.gas_meter.consume(expansion_cost)?;
                
                let data = self.context.memory.read_bytes(offset_usize, size_usize)?;
                self.context.event_logger.log(self.context.address, vec![], Bytes::new(data));
            }
            Opcode::Log1 => {
                let topic0 = self.context.stack.pop()?;
                let offset = self.context.stack.pop()?;
                let size = self.context.stack.pop()?;
                let offset_usize = offset.to_u64() as usize;
                let size_usize = size.to_u64() as usize;
                
                // Calculate memory expansion cost
                let new_size = offset_usize + size_usize;
                let expansion_cost = self.context.gas_meter.memory_expansion_cost(
                    self.context.memory.size(), 
                    new_size
                );
                self.context.gas_meter.consume(expansion_cost)?;
                
                let data = self.context.memory.read_bytes(offset_usize, size_usize)?;
                let topic0_bytes = topic0.to_bytes_be();
                let mut topic0_array = [0u8; 32];
                topic0_array.copy_from_slice(&topic0_bytes[..32]);
                let topics = vec![Hash::new(topic0_array)];
                self.context.event_logger.log(self.context.address, topics, Bytes::new(data));
            }
            Opcode::Log2 => {
                let topic1 = self.context.stack.pop()?;
                let topic0 = self.context.stack.pop()?;
                let offset = self.context.stack.pop()?;
                let size = self.context.stack.pop()?;
                let offset_usize = offset.to_u64() as usize;
                let size_usize = size.to_u64() as usize;
                
                // Calculate memory expansion cost
                let new_size = offset_usize + size_usize;
                let expansion_cost = self.context.gas_meter.memory_expansion_cost(
                    self.context.memory.size(), 
                    new_size
                );
                self.context.gas_meter.consume(expansion_cost)?;
                
                let data = self.context.memory.read_bytes(offset_usize, size_usize)?;
                let topic0_bytes = topic0.to_bytes_be();
                let topic1_bytes = topic1.to_bytes_be();
                let mut topic0_array = [0u8; 32];
                let mut topic1_array = [0u8; 32];
                topic0_array.copy_from_slice(&topic0_bytes[..32]);
                topic1_array.copy_from_slice(&topic1_bytes[..32]);
                let topics = vec![
                    Hash::new(topic0_array),
                    Hash::new(topic1_array),
                ];
                self.context.event_logger.log(self.context.address, topics, Bytes::new(data));
            }
            Opcode::Log3 => {
                let topic2 = self.context.stack.pop()?;
                let topic1 = self.context.stack.pop()?;
                let topic0 = self.context.stack.pop()?;
                let offset = self.context.stack.pop()?;
                let size = self.context.stack.pop()?;
                let offset_usize = offset.to_u64() as usize;
                let size_usize = size.to_u64() as usize;
                
                // Calculate memory expansion cost
                let new_size = offset_usize + size_usize;
                let expansion_cost = self.context.gas_meter.memory_expansion_cost(
                    self.context.memory.size(), 
                    new_size
                );
                self.context.gas_meter.consume(expansion_cost)?;
                
                let data = self.context.memory.read_bytes(offset_usize, size_usize)?;
                let topic0_bytes = topic0.to_bytes_be();
                let topic1_bytes = topic1.to_bytes_be();
                let topic2_bytes = topic2.to_bytes_be();
                let mut topic0_array = [0u8; 32];
                let mut topic1_array = [0u8; 32];
                let mut topic2_array = [0u8; 32];
                topic0_array.copy_from_slice(&topic0_bytes[..32]);
                topic1_array.copy_from_slice(&topic1_bytes[..32]);
                topic2_array.copy_from_slice(&topic2_bytes[..32]);
                let topics = vec![
                    Hash::new(topic0_array),
                    Hash::new(topic1_array),
                    Hash::new(topic2_array),
                ];
                self.context.event_logger.log(self.context.address, topics, Bytes::new(data));
            }
            Opcode::Log4 => {
                let topic3 = self.context.stack.pop()?;
                let topic2 = self.context.stack.pop()?;
                let topic1 = self.context.stack.pop()?;
                let topic0 = self.context.stack.pop()?;
                let offset = self.context.stack.pop()?;
                let size = self.context.stack.pop()?;
                let offset_usize = offset.to_u64() as usize;
                let size_usize = size.to_u64() as usize;
                
                // Calculate memory expansion cost
                let new_size = offset_usize + size_usize;
                let expansion_cost = self.context.gas_meter.memory_expansion_cost(
                    self.context.memory.size(), 
                    new_size
                );
                self.context.gas_meter.consume(expansion_cost)?;
                
                let data = self.context.memory.read_bytes(offset_usize, size_usize)?;
                let topic0_bytes = topic0.to_bytes_be();
                let topic1_bytes = topic1.to_bytes_be();
                let topic2_bytes = topic2.to_bytes_be();
                let topic3_bytes = topic3.to_bytes_be();
                let mut topic0_array = [0u8; 32];
                let mut topic1_array = [0u8; 32];
                let mut topic2_array = [0u8; 32];
                let mut topic3_array = [0u8; 32];
                topic0_array.copy_from_slice(&topic0_bytes[..32]);
                topic1_array.copy_from_slice(&topic1_bytes[..32]);
                topic2_array.copy_from_slice(&topic2_bytes[..32]);
                topic3_array.copy_from_slice(&topic3_bytes[..32]);
                let topics = vec![
                    Hash::new(topic0_array),
                    Hash::new(topic1_array),
                    Hash::new(topic2_array),
                    Hash::new(topic3_array),
                ];
                self.context.event_logger.log(self.context.address, topics, Bytes::new(data));
            }

            // Return operations
            Opcode::Return => {
                let offset = self.context.stack.pop()?;
                let size = self.context.stack.pop()?;
                let offset_usize = offset.to_u64() as usize;
                let size_usize = size.to_u64() as usize;
                self.context.return_data = Bytes::new(self.context.memory.read_bytes(offset_usize, size_usize)?);
                self.context.halt(true, "RETURN instruction".to_string());
            }
            Opcode::Revert => {
                let offset = self.context.stack.pop()?;
                let size = self.context.stack.pop()?;
                let offset_usize = offset.to_u64() as usize;
                let size_usize = size.to_u64() as usize;
                self.context.return_data = Bytes::new(self.context.memory.read_bytes(offset_usize, size_usize)?);
                self.context.halt(false, "REVERT instruction".to_string());
            }

            _ => {
                return Err(ExecutionError::InvalidInstruction(format!("Unimplemented opcode: {:?}", opcode)));
            }
        }

        Ok(())
    }

    /// Convert Uint256 to signed i256
    fn uint256_to_signed(&self, value: &Uint256) -> i128 {
        let bytes = value.to_bytes_be();
        let mut result = 0i128;
        for &byte in &bytes[16..] {
            result = (result << 8) | (byte as i128);
        }
        result
    }

    /// Convert signed i128 to Uint256
    fn signed_to_uint256(&self, value: i128) -> Uint256 {
        let mut bytes = [0u8; 32];
        let value_bytes = value.to_be_bytes();
        bytes[16..].copy_from_slice(&value_bytes);
        Uint256::from_bytes_be(&bytes)
    }
}

/// Result of EVM execution
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// Whether execution was successful
    pub success: bool,
    /// Return data
    pub return_data: Bytes,
    /// Gas used
    pub gas_used: u64,
    /// Gas remaining
    pub gas_remaining: u64,
    /// Event logs
    pub logs: Vec<EventLog>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_arithmetic() {
        // Code: PUSH1 0x02 PUSH1 0x03 ADD STOP
        let code = Bytes::from(vec![0x60, 0x02, 0x60, 0x03, 0x01, 0x00]);
        let context = ExecutionContext::new(
            Address::zero(),
            Address::zero(),
            Uint256::zero(),
            Bytes::empty(),
            code,
            1000,
        );
        let mut executor = Executor::new(context);
        let result = executor.execute().unwrap();
        
        assert!(result.success);
        assert_eq!(result.return_data.len(), 0);
    }

    #[test]
    fn test_stack_operations() {
        // Code: PUSH1 0x01 PUSH1 0x02 DUP1 ADD STOP
        let code = Bytes::from(vec![0x60, 0x01, 0x60, 0x02, 0x80, 0x01, 0x00]);
        let context = ExecutionContext::new(
            Address::zero(),
            Address::zero(),
            Uint256::zero(),
            Bytes::empty(),
            code,
            1000,
        );
        let mut executor = Executor::new(context);
        let result = executor.execute().unwrap();
        
        assert!(result.success);
    }

    #[test]
    fn test_memory_operations() {
        // Code: PUSH1 0x00 PUSH1 0x42 MSTORE8 PUSH1 0x00 MLOAD STOP
        let code = Bytes::from(vec![0x60, 0x00, 0x60, 0x42, 0x53, 0x60, 0x00, 0x51, 0x00]);
        let context = ExecutionContext::new(
            Address::zero(),
            Address::zero(),
            Uint256::zero(),
            Bytes::empty(),
            code,
            1000,
        );
        let mut executor = Executor::new(context);
        let result = executor.execute().unwrap();
        
        assert!(result.success);
    }
}
