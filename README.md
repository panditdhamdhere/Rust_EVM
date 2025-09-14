# EVM in Rust

An advanced implementation of the Ethereum Virtual Machine (EVM) written in Rust. This project provides a comprehensive EVM implementation including stack operations, memory management, storage, gas metering, opcode execution, debugging capabilities, and performance profiling.

## Features

### Core Components

- **Data Types**: Address, Uint256, Bytes, Hash with enhanced validation
- **Stack**: LIFO data structure with maximum 1024 items and overflow protection
- **Memory**: Expandable byte array for temporary storage with bounds checking
- **Storage**: Account state management with balance, nonce, code, and storage
- **Gas Metering**: Advanced gas consumption tracking with dynamic cost calculations
- **Opcodes**: Comprehensive EVM instruction set implementation (40+ opcodes)
- **Debug System**: Complete debugging infrastructure with execution tracing
- **Performance Profiling**: Gas analysis and execution monitoring

### Implemented Opcodes

#### Arithmetic Operations
- `ADD`, `SUB`, `MUL`, `DIV`, `MOD`, `EXP`

#### Comparison Operations
- `LT`, `GT`, `SLT`, `SGT`, `EQ`, `ISZERO`

#### Bitwise Operations
- `AND`, `OR`, `XOR`, `NOT`, `BYTE`, `SHL`, `SHR`

#### Cryptographic Operations
- `SHA3` (Keccak256 hashing)

#### Stack Operations
- `PUSH1`-`PUSH32`, `POP`, `DUP1`-`DUP16`, `SWAP1`-`SWAP16`

#### Memory Operations
- `MLOAD`, `MSTORE`, `MSTORE8`, `MSIZE`

#### Storage Operations
- `SLOAD`, `SSTORE`

#### Environmental Information
- `ADDRESS`, `CALLER`, `CALLVALUE`, `CALLDATASIZE`, `CALLDATALOAD`, `CODESIZE`, `CODECOPY`, `BALANCE`

#### Control Flow
- `JUMP`, `JUMPI`, `PC`, `JUMPDEST`

#### System Operations
- `STOP`, `RETURN`, `REVERT`

## Project Structure

```
src/
├── types/           # Core data types with enhanced validation
│   ├── address.rs   # Ethereum address (20 bytes)
│   ├── hash.rs      # Hash type (32 bytes)
│   ├── uint256.rs   # 256-bit unsigned integer with safe conversions
│   └── bytes.rs     # Variable-length byte array
├── stack/           # EVM stack implementation with overflow protection
├── memory/          # EVM memory management with bounds checking
├── storage/         # Account state and storage management
├── opcodes/         # EVM opcode definitions (40+ opcodes)
├── gas/             # Advanced gas metering and cost calculations
├── executor/        # Enhanced EVM execution engine with validation
├── debug/           # Debug system with tracing and profiling
└── main.rs          # Comprehensive examples and demonstrations
```

## Usage

### Running the Examples

```bash
cargo run
```

This will execute several example programs demonstrating:
1. Simple arithmetic operations
2. Stack manipulation
3. Memory operations
4. Comparison operations
5. Bitwise operations
6. SHA3 cryptographic operations
7. Advanced bitwise operations (BYTE)

### Running Tests

```bash
cargo test
```

This will run the comprehensive test suite with 30+ tests covering:
- Stack operations and overflow protection
- Memory management and bounds checking
- Storage operations and account management
- Gas metering and cost calculations
- Opcode execution and validation
- Debug system functionality
- Error handling and edge cases

### Performance

The implementation includes:
- **30+ Passing Tests**: Comprehensive test coverage
- **Memory Safety**: Bounds checking and overflow protection
- **Gas Efficiency**: Optimized gas calculations
- **Debug Capabilities**: Execution tracing and profiling
- **Error Handling**: Robust error reporting and validation

### Example EVM Code

#### Basic Arithmetic
```rust
use evm_rust::{
    types::{Address, Uint256, Bytes},
    executor::{Executor, ExecutionContext},
};

// Create a simple program: PUSH1 0x02 PUSH1 0x03 ADD STOP
let code = Bytes::from(vec![0x60, 0x02, 0x60, 0x03, 0x01, 0x00]);

let context = ExecutionContext::new(
    Address::zero(),
    Address::zero(),
    Uint256::zero(),
    Bytes::empty(),
    code,
    1000, // gas limit
);

let mut executor = Executor::new(context);
let result = executor.execute().unwrap();

println!("Execution successful: {}", result.success);
println!("Gas used: {}", result.gas_used);
```

#### SHA3 Cryptographic Operation
```rust
// SHA3 operation: PUSH1 0x00 PUSH1 0x20 MSTORE PUSH1 0x00 PUSH1 0x20 SHA3 STOP
let code = Bytes::from(vec![
    0x60, 0x00, // PUSH1 0x00 (memory offset)
    0x60, 0x20, // PUSH1 0x20 (value to store)
    0x52,       // MSTORE (store in memory)
    0x60, 0x00, // PUSH1 0x00 (offset for SHA3)
    0x60, 0x20, // PUSH1 0x20 (size for SHA3)
    0x20,       // SHA3
    0x00        // STOP
]);

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
```

#### Debug and Profiling
```rust
use evm_rust::debug::{Debugger, GasAnalyzer};

let mut debugger = Debugger::new();
debugger.enable();
debugger.add_breakpoint(10); // Break at PC 10

let mut gas_analyzer = GasAnalyzer::new();
// Record gas usage during execution
```

## Dependencies

- `num-bigint`: Big integer arithmetic for Uint256 with serde support
- `sha3`: Cryptographic hashing (Keccak256)
- `serde`: Serialization support with derive macros
- `thiserror`: Comprehensive error handling
- `hex`: Hex encoding/decoding
- `log` & `env_logger`: Logging infrastructure
- `indexmap`: Enhanced HashMap with insertion order

## Current Status

This is an advanced implementation with comprehensive EVM functionality including:

✅ **Implemented Features:**
- 40+ EVM opcodes including SHA3, bitwise operations, and environmental info
- Complete stack, memory, and storage management
- Advanced gas metering with dynamic cost calculations
- Debug system with execution tracing and breakpoints
- Performance profiling and gas analysis
- Comprehensive error handling and validation
- 30+ passing tests covering all major components

## Limitations

This implementation does not yet include:

- Advanced contract operations (CALL, CREATE, DELEGATECALL, etc.)
- Event logging (LOG operations)
- Complete gas cost calculations for all opcodes
- Network integration and RPC interface
- Blockchain state management
- Transaction processing and validation

## Future Enhancements

To extend this implementation further, you could add:

1. **Advanced Contract Operations**: Implement CALL, CREATE, DELEGATECALL, STATICCALL
2. **Event Logging**: Add LOG0-LOG4 operations for event emission
3. **More Cryptographic Operations**: Add RIPEMD160, ECRECOVER, etc.
4. **Advanced Gas Calculations**: Implement dynamic gas costs for all opcodes
5. **Network Integration**: Add RPC interface and blockchain connectivity
6. **State Management**: Implement complete blockchain state handling
7. **Transaction Processing**: Add transaction validation and processing
8. **Optimizations**: Performance improvements and memory optimization
9. **Precompiled Contracts**: Implement Ethereum precompiled contracts
10. **Multi-threading**: Parallel execution support for better performance

## License

This project is for educational purposes. Feel free to use and modify as needed.

## Contributing

This is a learning project, but suggestions and improvements are welcome!
