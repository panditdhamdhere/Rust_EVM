use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpcodeError {
    #[error("Invalid opcode: 0x{opcode:02x}")]
    InvalidOpcode { opcode: u8 },
    #[error("Stack underflow")]
    StackUnderflow,
    #[error("Stack overflow")]
    StackOverflow,
    #[error("Invalid jump destination")]
    InvalidJumpDestination,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Modulo by zero")]
    ModuloByZero,
}

/// EVM opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    // Stop and arithmetic operations
    Stop = 0x00,
    Add = 0x01,
    Mul = 0x02,
    Sub = 0x03,
    Div = 0x04,
    Sdiv = 0x05,
    Mod = 0x06,
    Smod = 0x07,
    Addmod = 0x08,
    Mulmod = 0x09,
    Exp = 0x0a,
    Signextend = 0x0b,

    // Comparison & bitwise logic operations
    Lt = 0x10,
    Gt = 0x11,
    Slt = 0x12,
    Sgt = 0x13,
    Eq = 0x14,
    Iszero = 0x15,
    And = 0x16,
    Or = 0x17,
    Xor = 0x18,
    Not = 0x19,
    Byte = 0x1a,
    Shl = 0x1b,
    Shr = 0x1c,
    Sar = 0x1d,

    // SHA3
    Sha3 = 0x20,

    // Environmental information
    Address = 0x30,
    Balance = 0x31,
    Origin = 0x32,
    Caller = 0x33,
    Callvalue = 0x34,
    Calldataload = 0x35,
    Calldatasize = 0x36,
    Calldatacopy = 0x37,
    Codesize = 0x38,
    Codecopy = 0x39,
    Gasprice = 0x3a,
    Extcodesize = 0x3b,
    Extcodecopy = 0x3c,
    Returndatasize = 0x3d,
    Returndatacopy = 0x3e,
    Extcodehash = 0x3f,

    // Block information
    Blockhash = 0x40,
    Coinbase = 0x41,
    Timestamp = 0x42,
    Number = 0x43,
    Difficulty = 0x44,
    Gaslimit = 0x45,
    Chainid = 0x46,
    Selfbalance = 0x47,

    // Storage and memory operations
    Pop = 0x50,
    Mload = 0x51,
    Mstore = 0x52,
    Mstore8 = 0x53,
    Sload = 0x54,
    Sstore = 0x55,
    Msize = 0x59,

    // Push operations
    Push1 = 0x60,
    Push2 = 0x61,
    Push3 = 0x62,
    Push4 = 0x63,
    Push5 = 0x64,
    Push6 = 0x65,
    Push7 = 0x66,
    Push8 = 0x67,
    Push9 = 0x68,
    Push10 = 0x69,
    Push11 = 0x6a,
    Push12 = 0x6b,
    Push13 = 0x6c,
    Push14 = 0x6d,
    Push15 = 0x6e,
    Push16 = 0x6f,
    Push17 = 0x70,
    Push18 = 0x71,
    Push19 = 0x72,
    Push20 = 0x73,
    Push21 = 0x74,
    Push22 = 0x75,
    Push23 = 0x76,
    Push24 = 0x77,
    Push25 = 0x78,
    Push26 = 0x79,
    Push27 = 0x7a,
    Push28 = 0x7b,
    Push29 = 0x7c,
    Push30 = 0x7d,
    Push31 = 0x7e,
    Push32 = 0x7f,

    // Duplicate operations
    Dup1 = 0x80,
    Dup2 = 0x81,
    Dup3 = 0x82,
    Dup4 = 0x83,
    Dup5 = 0x84,
    Dup6 = 0x85,
    Dup7 = 0x86,
    Dup8 = 0x87,
    Dup9 = 0x88,
    Dup10 = 0x89,
    Dup11 = 0x8a,
    Dup12 = 0x8b,
    Dup13 = 0x8c,
    Dup14 = 0x8d,
    Dup15 = 0x8e,
    Dup16 = 0x8f,

    // Exchange operations
    Swap1 = 0x90,
    Swap2 = 0x91,
    Swap3 = 0x92,
    Swap4 = 0x93,
    Swap5 = 0x94,
    Swap6 = 0x95,
    Swap7 = 0x96,
    Swap8 = 0x97,
    Swap9 = 0x98,
    Swap10 = 0x99,
    Swap11 = 0x9a,
    Swap12 = 0x9b,
    Swap13 = 0x9c,
    Swap14 = 0x9d,
    Swap15 = 0x9e,
    Swap16 = 0x9f,

    // Logging operations
    Log0 = 0xa0,
    Log1 = 0xa1,
    Log2 = 0xa2,
    Log3 = 0xa3,
    Log4 = 0xa4,

    // System operations
    Create = 0xf0,
    Call = 0xf1,
    Callcode = 0xf2,
    Return = 0xf3,
    Delegatecall = 0xf4,
    Create2 = 0xf5,
    Staticcall = 0xfa,
    Revert = 0xfd,
    Selfdestruct = 0xff,

    // Control flow
    Jump = 0x56,
    Jumpi = 0x57,
    Pc = 0x58,
    Jumpdest = 0x5b,
}

impl Opcode {
    /// Convert a byte to an opcode
    pub fn from_byte(byte: u8) -> Result<Self, OpcodeError> {
        match byte {
            0x00 => Ok(Opcode::Stop),
            0x01 => Ok(Opcode::Add),
            0x02 => Ok(Opcode::Mul),
            0x03 => Ok(Opcode::Sub),
            0x04 => Ok(Opcode::Div),
            0x05 => Ok(Opcode::Sdiv),
            0x06 => Ok(Opcode::Mod),
            0x07 => Ok(Opcode::Smod),
            0x08 => Ok(Opcode::Addmod),
            0x09 => Ok(Opcode::Mulmod),
            0x0a => Ok(Opcode::Exp),
            0x0b => Ok(Opcode::Signextend),
            0x10 => Ok(Opcode::Lt),
            0x11 => Ok(Opcode::Gt),
            0x12 => Ok(Opcode::Slt),
            0x13 => Ok(Opcode::Sgt),
            0x14 => Ok(Opcode::Eq),
            0x15 => Ok(Opcode::Iszero),
            0x16 => Ok(Opcode::And),
            0x17 => Ok(Opcode::Or),
            0x18 => Ok(Opcode::Xor),
            0x19 => Ok(Opcode::Not),
            0x1a => Ok(Opcode::Byte),
            0x1b => Ok(Opcode::Shl),
            0x1c => Ok(Opcode::Shr),
            0x1d => Ok(Opcode::Sar),
            0x20 => Ok(Opcode::Sha3),
            0x30 => Ok(Opcode::Address),
            0x31 => Ok(Opcode::Balance),
            0x32 => Ok(Opcode::Origin),
            0x33 => Ok(Opcode::Caller),
            0x34 => Ok(Opcode::Callvalue),
            0x35 => Ok(Opcode::Calldataload),
            0x36 => Ok(Opcode::Calldatasize),
            0x37 => Ok(Opcode::Calldatacopy),
            0x38 => Ok(Opcode::Codesize),
            0x39 => Ok(Opcode::Codecopy),
            0x3a => Ok(Opcode::Gasprice),
            0x3b => Ok(Opcode::Extcodesize),
            0x3c => Ok(Opcode::Extcodecopy),
            0x3d => Ok(Opcode::Returndatasize),
            0x3e => Ok(Opcode::Returndatacopy),
            0x3f => Ok(Opcode::Extcodehash),
            0x40 => Ok(Opcode::Blockhash),
            0x41 => Ok(Opcode::Coinbase),
            0x42 => Ok(Opcode::Timestamp),
            0x43 => Ok(Opcode::Number),
            0x44 => Ok(Opcode::Difficulty),
            0x45 => Ok(Opcode::Gaslimit),
            0x46 => Ok(Opcode::Chainid),
            0x47 => Ok(Opcode::Selfbalance),
            0x50 => Ok(Opcode::Pop),
            0x51 => Ok(Opcode::Mload),
            0x52 => Ok(Opcode::Mstore),
            0x53 => Ok(Opcode::Mstore8),
            0x54 => Ok(Opcode::Sload),
            0x55 => Ok(Opcode::Sstore),
            0x56 => Ok(Opcode::Jump),
            0x57 => Ok(Opcode::Jumpi),
            0x58 => Ok(Opcode::Pc),
            0x59 => Ok(Opcode::Msize),
            0x5b => Ok(Opcode::Jumpdest),
            0x60..=0x7f => Ok(Opcode::Push1), // Will be handled specially
            0x80..=0x8f => Ok(Opcode::Dup1), // Will be handled specially
            0x90..=0x9f => Ok(Opcode::Swap1), // Will be handled specially
            0xa0 => Ok(Opcode::Log0),
            0xa1 => Ok(Opcode::Log1),
            0xa2 => Ok(Opcode::Log2),
            0xa3 => Ok(Opcode::Log3),
            0xa4 => Ok(Opcode::Log4),
            0xf0 => Ok(Opcode::Create),
            0xf1 => Ok(Opcode::Call),
            0xf2 => Ok(Opcode::Callcode),
            0xf3 => Ok(Opcode::Return),
            0xf4 => Ok(Opcode::Delegatecall),
            0xf5 => Ok(Opcode::Create2),
            0xfa => Ok(Opcode::Staticcall),
            0xfd => Ok(Opcode::Revert),
            0xff => Ok(Opcode::Selfdestruct),
            _ => Err(OpcodeError::InvalidOpcode { opcode: byte }),
        }
    }

    /// Convert opcode to byte
    pub fn to_byte(self) -> u8 {
        self as u8
    }

    /// Get the number of bytes this opcode pushes to the stack
    pub fn push_size(self) -> usize {
        let byte = self.to_byte();
        if byte >= 0x60 && byte <= 0x7f {
            (byte - 0x60 + 1) as usize
        } else {
            0
        }
    }

    /// Get the number of items this opcode pops from the stack
    pub fn pop_count(self) -> usize {
        match self {
            // Arithmetic operations
            Opcode::Add | Opcode::Mul | Opcode::Sub | Opcode::Div | Opcode::Sdiv |
            Opcode::Mod | Opcode::Smod | Opcode::Addmod | Opcode::Mulmod | Opcode::Exp |
            Opcode::Signextend => 2,
            Opcode::Iszero | Opcode::Not => 1,
            Opcode::Lt | Opcode::Gt | Opcode::Slt | Opcode::Sgt | Opcode::Eq |
            Opcode::And | Opcode::Or | Opcode::Xor | Opcode::Byte | Opcode::Shl |
            Opcode::Shr | Opcode::Sar => 2,
            Opcode::Sha3 => 2,
            Opcode::Calldataload | Opcode::Sload | Opcode::Mload => 1,
            Opcode::Calldatacopy | Opcode::Codecopy | Opcode::Extcodecopy |
            Opcode::Returndatacopy => 3,
            Opcode::Mstore | Opcode::Mstore8 | Opcode::Sstore => 2,
            Opcode::Pop => 1,
            Opcode::Jump => 1,
            Opcode::Jumpi => 2,
            Opcode::Log0 => 2,
            Opcode::Log1 => 3,
            Opcode::Log2 => 4,
            Opcode::Log3 => 5,
            Opcode::Log4 => 6,
            Opcode::Create | Opcode::Create2 => 3,
            Opcode::Call | Opcode::Callcode | Opcode::Delegatecall | Opcode::Staticcall => 7,
            Opcode::Return | Opcode::Revert => 2,
            Opcode::Selfdestruct => 1,
            _ => 0,
        }
    }

    /// Check if this is a push opcode
    pub fn is_push(self) -> bool {
        let byte = self.to_byte();
        byte >= 0x60 && byte <= 0x7f
    }

    /// Check if this is a dup opcode
    pub fn is_dup(self) -> bool {
        let byte = self.to_byte();
        byte >= 0x80 && byte <= 0x8f
    }

    /// Check if this is a swap opcode
    pub fn is_swap(self) -> bool {
        let byte = self.to_byte();
        byte >= 0x90 && byte <= 0x9f
    }

    /// Get the dup depth (0-based)
    pub fn dup_depth(self) -> usize {
        if self.is_dup() {
            (self.to_byte() - 0x80) as usize
        } else {
            0
        }
    }

    /// Get the swap depth (0-based)
    pub fn swap_depth(self) -> usize {
        if self.is_swap() {
            (self.to_byte() - 0x90) as usize
        } else {
            0
        }
    }

    /// Get the push size for push opcodes
    pub fn get_push_size(self) -> usize {
        if self.is_push() {
            (self.to_byte() - 0x60 + 1) as usize
        } else {
            0
        }
    }
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_from_byte() {
        assert_eq!(Opcode::from_byte(0x00).unwrap(), Opcode::Stop);
        assert_eq!(Opcode::from_byte(0x01).unwrap(), Opcode::Add);
        assert_eq!(Opcode::from_byte(0x60).unwrap(), Opcode::Push1);
        assert_eq!(Opcode::from_byte(0x7f).unwrap(), Opcode::Push1); // Will be handled specially
        assert!(Opcode::from_byte(0xff).is_ok());
        assert!(Opcode::from_byte(0xfe).is_err());
    }

    #[test]
    fn test_opcode_properties() {
        assert_eq!(Opcode::Add.pop_count(), 2);
        assert_eq!(Opcode::Iszero.pop_count(), 1);
        assert_eq!(Opcode::Pop.pop_count(), 1);
        assert_eq!(Opcode::Push1.pop_count(), 0);
        assert_eq!(Opcode::Dup1.pop_count(), 0);
        assert_eq!(Opcode::Swap1.pop_count(), 0);
    }

    #[test]
    fn test_push_opcodes() {
        assert!(Opcode::Push1.is_push());
        assert!(Opcode::Push32.is_push());
        assert!(!Opcode::Add.is_push());
        
        assert_eq!(Opcode::Push1.get_push_size(), 1);
        assert_eq!(Opcode::Push32.get_push_size(), 32);
    }

    #[test]
    fn test_dup_swap_opcodes() {
        assert!(Opcode::Dup1.is_dup());
        assert!(Opcode::Dup16.is_dup());
        assert!(!Opcode::Add.is_dup());
        
        assert!(Opcode::Swap1.is_swap());
        assert!(Opcode::Swap16.is_swap());
        assert!(!Opcode::Add.is_swap());
        
        assert_eq!(Opcode::Dup1.dup_depth(), 0);
        assert_eq!(Opcode::Dup16.dup_depth(), 15);
        
        assert_eq!(Opcode::Swap1.swap_depth(), 0);
        assert_eq!(Opcode::Swap16.swap_depth(), 15);
    }
}
