# EVM in Rust

A basic implementation of the Ethereum Virtual Machine (EVM) written in Rust. This project demonstrates the core components of an EVM including stack operations, memory management, storage, gas metering, and opcode execution.

## Features

### Core Components

- **Data Types**: Address, Uint256, Bytes, Hash
- **Stack**: LIFO data structure with maximum 1024 items
- **Memory**: Expandable byte array for temporary storage
- **Storage**: Account state management with balance, nonce, code, and storage
- **Gas Metering**: Gas consumption tracking and limits
- **Opcodes**: Basic EVM instruction set implementation

### Implemented Opcodes

#### Arithmetic Operations
- `ADD`, `SUB`, `MUL`, `DIV`, `MOD`, `EXP`

#### Comparison Operations
- `LT`, `GT`, `EQ`, `ISZERO`

#### Bitwise Operations
- `AND`, `OR`, `XOR`, `NOT`

#### Stack Operations
- `PUSH1`-`PUSH32`, `POP`, `DUP1`-`DUP16`, `SWAP1`-`SWAP16`

#### Memory Operations
- `MLOAD`, `MSTORE`, `MSTORE8`, `MSIZE`

#### Storage Operations
- `SLOAD`, `SSTORE`

#### Environmental Information
- `ADDRESS`, `CALLER`, `CALLVALUE`, `CALLDATASIZE`, `CALLDATALOAD`, `CODESIZE`

#### Control Flow
- `JUMP`, `JUMPI`, `PC`, `JUMPDEST`

#### System Operations
- `STOP`, `RETURN`, `REVERT`

## Project Structure

```
src/
├── types/           # Core data types
│   ├── address.rs   # Ethereum address (20 bytes)
│   ├── hash.rs      # Hash type (32 bytes)
│   ├── uint256.rs   # 256-bit unsigned integer
│   └── bytes.rs     # Variable-length byte array
├── stack/           # EVM stack implementation
├── memory/          # EVM memory management
├── storage/         # Account state and storage
├── opcodes/         # EVM opcode definitions
├── gas/             # Gas metering and costs
├── executor/        # EVM execution engine
└── main.rs          # Example usage and demonstrations
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

### Running Tests

```bash
cargo test
```

### Example EVM Code

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

## Dependencies

- `num-bigint`: Big integer arithmetic for Uint256
- `sha3`: Cryptographic hashing
- `serde`: Serialization support
- `thiserror`: Error handling
- `hex`: Hex encoding/decoding

## Limitations

This is a basic implementation for educational purposes. It does not include:

- Advanced opcodes (SHA3, CALL, CREATE, etc.)
- Cryptographic operations
- Contract creation and calling
- Event logging
- Complete gas cost calculations
- Network integration

## Future Enhancements

To extend this implementation, you could add:

1. **More Opcodes**: Implement remaining EVM opcodes
2. **Cryptographic Operations**: Add SHA3, RIPEMD160, etc.
3. **Contract Operations**: Implement CREATE, CALL, DELEGATECALL
4. **Event Logging**: Add LOG operations
5. **Advanced Gas Calculations**: Implement dynamic gas costs
6. **Network Integration**: Add RPC interface
7. **State Management**: Implement blockchain state
8. **Transaction Processing**: Add transaction validation

## License

This project is for educational purposes. Feel free to use and modify as needed.

## Contributing

This is a learning project, but suggestions and improvements are welcome!
