# EVM Rust CLI Guide

A comprehensive guide to using the EVM Rust command-line interface.

## Table of Contents

1. [Installation](#installation)
2. [Basic Usage](#basic-usage)
3. [Commands](#commands)
4. [Examples](#examples)
5. [Advanced Features](#advanced-features)
6. [Troubleshooting](#troubleshooting)

## Installation

### Prerequisites

- Rust 1.70+ installed
- Git (for cloning the repository)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/panditdhamdhere/Rust_EVM
cd evm-rust

# Build the project
cargo build --release

# The binary will be available at target/release/evm-rust
```

### Quick Start

```bash
# Run with help to see all available commands
./target/release/evm-rust --help

# Or if you have cargo installed
cargo run -- --help
```

## Basic Usage

The EVM Rust CLI provides several commands for interacting with the Ethereum Virtual Machine:

```bash
evm-rust <COMMAND> [OPTIONS]
```

### Global Options

- `--help`: Show help information
- `--version`: Show version information

## Commands

### 1. Execute Bytecode

Execute EVM bytecode directly.

```bash
evm-rust execute [OPTIONS]
```

#### Options

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `--code <HEX>` | Bytecode to execute (hex string) | Required | `--code "6002600301"` |
| `--gas-limit <LIMIT>` | Gas limit for execution | `1000000` | `--gas-limit 500000` |
| `--debug` | Enable debug output | `false` | `--debug` |
| `--trace` | Enable basic tracing | `false` | `--trace` |
| `--detailed-trace` | Enable detailed execution tracing | `false` | `--detailed-trace` |
| `--caller <ADDRESS>` | Caller address (hex) | `0x0000...0000` | `--caller "0x1234..."` |
| `--address <ADDRESS>` | Contract address (hex) | `0x0000...0000` | `--address "0x5678..."` |
| `--value <WEI>` | Call value in wei | `0` | `--value "1000000000000000000"` |
| `--input <HEX>` | Input data (hex) | `""` | `--input "0x1234"` |
| `--no-validate` | Disable validation checks | `false` | `--no-validate` |
| `--export-trace <FILE>` | Export trace to file | None | `--export-trace trace.json` |

#### Examples

**Basic execution:**
```bash
evm-rust execute --code "6002600301"
```

**With custom gas limit:**
```bash
evm-rust execute --code "6002600301" --gas-limit 500000
```

**With debug output:**
```bash
evm-rust execute --code "6002600301" --debug
```

**With detailed tracing:**
```bash
evm-rust execute --code "6002600301" --detailed-trace
```

**Export execution trace:**
```bash
evm-rust execute --code "6002600301" --export-trace execution.json
```

**With custom context:**
```bash
evm-rust execute \
  --code "6002600301" \
  --caller "0x1234567890123456789012345678901234567890" \
  --address "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd" \
  --value "1000000000000000000" \
  --input "0x1234"
```

### 2. Run Examples

Execute predefined EVM examples.

```bash
evm-rust examples [OPTIONS]
```

#### Options

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `--number <NUM>` | Run specific example (1-10) | None | `--number 5` |
| `--list` | List all available examples | `false` | `--list` |

#### Examples

**List all examples:**
```bash
evm-rust examples --list
```

**Run specific example:**
```bash
evm-rust examples --number 3
```

**Run all examples:**
```bash
evm-rust examples
```

### 3. Interactive Shell

Start an interactive EVM shell (placeholder).

```bash
evm-rust shell [OPTIONS]
```

#### Options

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `--gas-limit <LIMIT>` | Gas limit for shell execution | `1000000` | `--gas-limit 500000` |

#### Examples

```bash
evm-rust shell
evm-rust shell --gas-limit 2000000
```

### 4. Show Information

Display EVM information and statistics.

```bash
evm-rust info [OPTIONS]
```

#### Options

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `--opcodes` | Show supported opcodes | `false` | `--opcodes` |
| `--gas-costs` | Show gas cost information | `false` | `--gas-costs` |
| `--validation` | Show validation limits | `false` | `--validation` |

#### Examples

**Show all information:**
```bash
evm-rust info --opcodes --gas-costs --validation
```

**Show only opcodes:**
```bash
evm-rust info --opcodes
```

**Show validation limits:**
```bash
evm-rust info --validation
```

## Examples

### Example 1: Basic Arithmetic

```bash
# Add 2 + 3 = 5
evm-rust execute --code "6002600301" --debug
```

**Bytecode breakdown:**
- `60 02`: PUSH1 0x02 (push 2 to stack)
- `60 03`: PUSH1 0x03 (push 3 to stack)
- `01`: ADD (pop 2 values, add them, push result)

### Example 2: Stack Operations

```bash
# Demonstrate stack operations
evm-rust execute --code "600160026003808080" --detailed-trace
```

### Example 3: Memory Operations

```bash
# Store and load from memory
evm-rust execute --code "600260005260016000f3" --debug
```

### Example 4: SHA3 Hashing

```bash
# Hash data using SHA3
evm-rust execute --code "60016000536020" --detailed-trace
```

### Example 5: Event Logging

```bash
# Emit a LOG0 event
evm-rust execute --code "60016000536020a0" --debug
```

## Advanced Features

### Validation System

The EVM includes a comprehensive validation system that checks:

- **Bytecode validation**: Ensures all opcodes are valid
- **Jump destination validation**: Verifies JUMP/JUMPI targets
- **Security checks**: Detects potential infinite loops and expensive opcode patterns
- **Address validation**: Ensures addresses are properly formatted
- **Gas limit validation**: Checks gas limits are within reasonable bounds
- **Value validation**: Prevents overflow attacks

**Disable validation:**
```bash
evm-rust execute --code "6002600301" --no-validate
```

### Execution Tracing

The EVM provides detailed execution tracing capabilities:

#### Basic Tracing
```bash
evm-rust execute --code "6002600301" --trace
```

#### Detailed Tracing
```bash
evm-rust execute --code "6002600301" --detailed-trace
```

**Trace output includes:**
- Step-by-step execution
- Stack state changes
- Memory modifications
- Storage changes
- Gas consumption per step
- Execution statistics

#### Export Traces

**JSON format (when Serialize is implemented):**
```bash
evm-rust execute --code "6002600301" --export-trace trace.json
```

**CSV format (fallback):**
```bash
evm-rust execute --code "6002600301" --export-trace trace.json
# Automatically exports as CSV if JSON fails
```

### Gas Analysis

The EVM tracks detailed gas consumption:

```bash
evm-rust execute --code "6002600301" --detailed-trace
```

**Gas information includes:**
- Total gas consumed
- Gas per opcode
- Gas efficiency metrics
- Memory expansion costs
- Storage operation costs

### Event Logging

The EVM supports Ethereum-style event logging:

```bash
evm-rust execute --code "60016000536020a0" --debug
```

**Event features:**
- LOG0, LOG1, LOG2, LOG3, LOG4 operations
- Topic and data logging
- Event filtering and analysis

### Block Context

The EVM provides access to blockchain context:

```bash
evm-rust execute --code "42" --debug  # BLOCKHASH
evm-rust execute --code "41" --debug  # COINBASE
evm-rust execute --code "42" --debug  # TIMESTAMP
```

**Available context:**
- Block number, timestamp, difficulty
- Gas limit, coinbase, chain ID
- Block hash, base fee
- Transaction context (gas price, origin, etc.)

## Troubleshooting

### Common Issues

#### 1. Invalid Bytecode Error

**Error:**
```
❌ Error: Invalid bytecode: Invalid opcode 0xfe at position 0
```

**Solution:**
- Ensure bytecode is valid hex
- Check that all opcodes are supported
- Use `--no-validate` to bypass validation (not recommended)

#### 2. Stack Underflow

**Error:**
```
❌ Error: Stack error: Stack underflow: not enough items on stack
```

**Solution:**
- Ensure sufficient values are pushed before operations
- Check bytecode logic for proper stack management

#### 3. Gas Limit Exceeded

**Error:**
```
❌ Error: Gas limit exceeded
```

**Solution:**
- Increase gas limit with `--gas-limit`
- Optimize bytecode to use less gas
- Check for infinite loops

#### 4. Validation Failed

**Error:**
```
❌ Error: Invalid gas limit: 50000000
```

**Solution:**
- Use gas limits between 21,000 and 30,000,000
- Check address formats (42 characters, starts with 0x)
- Ensure values are reasonable

### Debug Tips

#### 1. Use Debug Mode
```bash
evm-rust execute --code "YOUR_CODE" --debug
```

#### 2. Enable Detailed Tracing
```bash
evm-rust execute --code "YOUR_CODE" --detailed-trace
```

#### 3. Export Traces for Analysis
```bash
evm-rust execute --code "YOUR_CODE" --export-trace analysis.csv
```

#### 4. Check Validation Limits
```bash
evm-rust info --validation
```

### Performance Tips

1. **Use appropriate gas limits**: Start with 1M gas for simple operations
2. **Enable validation**: Always use validation unless debugging
3. **Export traces**: Use CSV export for large executions
4. **Batch operations**: Combine multiple operations in single execution

### Getting Help

1. **Command help:**
   ```bash
   evm-rust --help
   evm-rust execute --help
   ```

2. **Information commands:**
   ```bash
   evm-rust info --opcodes
   evm-rust info --gas-costs
   evm-rust info --validation
   ```

3. **Examples:**
   ```bash
   evm-rust examples --list
   evm-rust examples --number 1
   ```

## Contributing

To contribute to the EVM Rust project:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request


## Support

For support and questions:

- Create an issue on GitHub
- Check the documentation
- Review the examples
- Use the built-in help system

---

**Happy EVM coding! 🚀**
