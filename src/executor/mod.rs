use crate::{
    types::{Address, Uint256, Bytes},
    stack::{Stack, StackError},
    memory::{Memory, MemoryError},
    storage::{Storage, StorageError},
    gas::{GasMeter, GasError},
    opcodes::{Opcode, OpcodeError},
};
use thiserror::Error;

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

        Ok(ExecutionResult {
            success: self.context.success,
            return_data: self.context.return_data.clone(),
            gas_used: self.context.gas_meter.used(),
            gas_remaining: self.context.gas_meter.available(),
        })
    }

    /// Execute a single step
    pub fn step(&mut self) -> Result<(), ExecutionError> {
        let opcode_byte = self.context.current_instruction()?;
        let opcode = Opcode::from_byte(opcode_byte)?;

        // Handle push opcodes specially
        if opcode.is_push() {
            self.execute_push(opcode)?;
        } else {
            self.execute_opcode(opcode)?;
        }

        Ok(())
    }

    /// Execute a push opcode
    fn execute_push(&mut self, opcode: Opcode) -> Result<(), ExecutionError> {
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
            Opcode::Not => {
                let a = self.context.stack.pop()?;
                // NOT operation on 256-bit value (bitwise complement)
                let max_uint256 = Uint256::from_bytes_be(&[0xFF; 32]);
                let result = a ^ max_uint256;
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
                let value = self.context.memory.read_word(offset_usize)?;
                self.context.stack.push(value)?;
            }
            Opcode::Mstore => {
                let offset = self.context.stack.pop()?;
                let value = self.context.stack.pop()?;
                let offset_usize = offset.to_u64() as usize;
                self.context.memory.write_word(offset_usize, value)?;
            }
            Opcode::Mstore8 => {
                let offset = self.context.stack.pop()?;
                let value = self.context.stack.pop()?;
                let offset_usize = offset.to_u64() as usize;
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
