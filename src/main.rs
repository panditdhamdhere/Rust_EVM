use evm_rust::{
    types::{Address, Uint256, Bytes},
    executor::{Executor, ExecutionContext},
};

fn main() {
    env_logger::init();
    
    println!("EVM in Rust - Ethereum Virtual Machine Implementation");
    println!("=====================================================");
    
    // Example 1: Simple arithmetic
    println!("\n1. Simple Arithmetic Example:");
    println!("Code: PUSH1 0x02 PUSH1 0x03 ADD STOP");
    println!("Expected result: 5 (0x05)");
    
    let code = Bytes::from(vec![
        0x60, 0x02, // PUSH1 0x02
        0x60, 0x03, // PUSH1 0x03
        0x01,       // ADD
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
    match executor.execute() {
        Ok(result) => {
            println!("Execution successful: {}", result.success);
            println!("Gas used: {}", result.gas_used);
            println!("Gas remaining: {}", result.gas_remaining);
            if !result.logs.is_empty() {
                println!("Event logs: {}", result.logs.len());
                for (i, log) in result.logs.iter().enumerate() {
                    println!("  Log {}: {}", i, log);
                }
            }
        }
        Err(e) => {
            println!("Execution failed: {}", e);
        }
    }
    
    // Example 2: Stack operations
    println!("\n2. Stack Operations Example:");
    println!("Code: PUSH1 0x01 PUSH1 0x02 DUP1 ADD STOP");
    println!("Expected: Duplicate top item (0x02) and add to second item (0x01)");
    
    let code = Bytes::from(vec![
        0x60, 0x01, // PUSH1 0x01
        0x60, 0x02, // PUSH1 0x02
        0x80,       // DUP1 (duplicate top item)
        0x01,       // ADD
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
    match executor.execute() {
        Ok(result) => {
            println!("Execution successful: {}", result.success);
            println!("Gas used: {}", result.gas_used);
        }
        Err(e) => {
            println!("Execution failed: {}", e);
        }
    }
    
    // Example 3: Memory operations
    println!("\n3. Memory Operations Example:");
    println!("Code: PUSH1 0x00 PUSH1 0x42 MSTORE8 PUSH1 0x00 MLOAD STOP");
    println!("Expected: Store 0x42 in memory at offset 0, then load it back");
    
    let code = Bytes::from(vec![
        0x60, 0x00, // PUSH1 0x00 (memory offset)
        0x60, 0x42, // PUSH1 0x42 (value to store)
        0x53,       // MSTORE8 (store byte in memory)
        0x60, 0x00, // PUSH1 0x00 (memory offset to load from)
        0x51,       // MLOAD (load word from memory)
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
    match executor.execute() {
        Ok(result) => {
            println!("Execution successful: {}", result.success);
            println!("Gas used: {}", result.gas_used);
        }
        Err(e) => {
            println!("Execution failed: {}", e);
        }
    }
    
    // Example 4: Comparison operations
    println!("\n4. Comparison Operations Example:");
    println!("Code: PUSH1 0x05 PUSH1 0x03 GT STOP");
    println!("Expected: 1 (true) since 5 > 3");
    
    let code = Bytes::from(vec![
        0x60, 0x05, // PUSH1 0x05
        0x60, 0x03, // PUSH1 0x03
        0x11,       // GT (greater than)
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
    match executor.execute() {
        Ok(result) => {
            println!("Execution successful: {}", result.success);
            println!("Gas used: {}", result.gas_used);
        }
        Err(e) => {
            println!("Execution failed: {}", e);
        }
    }
    
    // Example 5: Bitwise operations
    println!("\n5. Bitwise Operations Example:");
    println!("Code: PUSH1 0x0F PUSH1 0x33 AND STOP");
    println!("Expected: 0x03 (0x0F & 0x33 = 0x03)");
    
    let code = Bytes::from(vec![
        0x60, 0x0F, // PUSH1 0x0F
        0x60, 0x33, // PUSH1 0x33
        0x16,       // AND
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
    match executor.execute() {
        Ok(result) => {
            println!("Execution successful: {}", result.success);
            println!("Gas used: {}", result.gas_used);
        }
        Err(e) => {
            println!("Execution failed: {}", e);
        }
    }
    
    // Example 6: SHA3 operation
    println!("\n6. SHA3 Operation Example:");
    println!("Code: PUSH1 0x00 PUSH1 0x20 MSTORE PUSH1 0x00 PUSH1 0x20 SHA3 STOP");
    println!("Expected: Calculate Keccak256 hash of 32 bytes of memory");
    
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
    match executor.execute() {
        Ok(result) => {
            println!("Execution successful: {}", result.success);
            println!("Gas used: {}", result.gas_used);
        }
        Err(e) => {
            println!("Execution failed: {}", e);
        }
    }

    // Example 7: Byte operation
    println!("\n7. Byte Operation Example:");
    println!("Code: PUSH4 0x12345678 PUSH1 0x01 BYTE POP STOP");
    println!("Expected: Extract byte at index 1 (0x56) and pop it");
    
    let code = Bytes::from(vec![
        0x63, 0x12, 0x34, 0x56, 0x78, // PUSH4 0x12345678
        0x60, 0x01,                   // PUSH1 0x01 (byte index)
        0x1a,                         // BYTE
        0x50,                         // POP (to balance the stack)
        0x00                          // STOP
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
    match executor.execute() {
        Ok(result) => {
            println!("Execution successful: {}", result.success);
            println!("Gas used: {}", result.gas_used);
        }
        Err(e) => {
            println!("Execution failed: {}", e);
        }
    }

    // Example 8: Event Logging
    println!("\n8. Event Logging Example:");
    println!("Code: PUSH1 0x00 PUSH1 0x20 MSTORE PUSH1 0x00 PUSH1 0x20 LOG0 STOP");
    println!("Expected: Log 32 bytes of memory data with no topics");
    
    // LOG0: PUSH1 0x00 PUSH1 0x20 MSTORE PUSH1 0x00 PUSH1 0x20 LOG0 STOP
    let log_code = Bytes::from(vec![
        0x60, 0x00, // PUSH1 0x00 (memory offset)
        0x60, 0x20, // PUSH1 0x20 (value to store)
        0x52,       // MSTORE (store in memory)
        0x60, 0x00, // PUSH1 0x00 (log offset)
        0x60, 0x20, // PUSH1 0x20 (log size)
        0xa0,       // LOG0
        0x00        // STOP
    ]);
    
    let context = ExecutionContext::new(
        Address::zero(),
        Address::zero(),
        Uint256::zero(),
        Bytes::empty(),
        log_code,
        1000,
    );
    
    let mut executor = Executor::new(context);
    match executor.execute() {
        Ok(result) => {
            println!("Execution successful: {}", result.success);
            println!("Gas used: {}", result.gas_used);
            println!("Gas remaining: {}", result.gas_remaining);
            if !result.logs.is_empty() {
                println!("Event logs: {}", result.logs.len());
                for (i, log) in result.logs.iter().enumerate() {
                    println!("  Log {}: {}", i, log);
                }
            }
        }
        Err(e) => {
            println!("Execution failed: {}", e);
        }
    }

    // Example 9: Block Information
    println!("\n9. Block Information Example:");
    println!("Code: TIMESTAMP NUMBER CHAINID STOP");
    println!("Expected: Push current timestamp, block number, and chain ID to stack");
    
    // Block info: TIMESTAMP NUMBER CHAINID STOP
    let block_code = Bytes::from(vec![
        0x42, // TIMESTAMP
        0x43, // NUMBER  
        0x46, // CHAINID
        0x00  // STOP
    ]);
    
    let context = ExecutionContext::new(
        Address::zero(),
        Address::zero(),
        Uint256::zero(),
        Bytes::empty(),
        block_code,
        1000,
    );
    
    let mut executor = Executor::new(context);
    match executor.execute() {
        Ok(result) => {
            println!("Execution successful: {}", result.success);
            println!("Gas used: {}", result.gas_used);
            println!("Gas remaining: {}", result.gas_remaining);
        }
        Err(e) => {
            println!("Execution failed: {}", e);
        }
    }

    // Example 10: Advanced Arithmetic Operations
    println!("\n10. Advanced Arithmetic Example:");
    println!("Code: PUSH1 0x05 PUSH1 0x03 PUSH1 0x02 ADDMOD STOP");
    println!("Expected: (5 + 3) mod 2 = 0");
    
    // Advanced arithmetic: PUSH1 0x05 PUSH1 0x03 PUSH1 0x02 ADDMOD STOP
    let advanced_code = Bytes::from(vec![
        0x60, 0x05, // PUSH1 0x05
        0x60, 0x03, // PUSH1 0x03
        0x60, 0x02, // PUSH1 0x02
        0x08,       // ADDMOD
        0x00        // STOP
    ]);
    
    let context = ExecutionContext::new(
        Address::zero(),
        Address::zero(),
        Uint256::zero(),
        Bytes::empty(),
        advanced_code,
        1000,
    );
    
    let mut executor = Executor::new(context);
    match executor.execute() {
        Ok(result) => {
            println!("Execution successful: {}", result.success);
            println!("Gas used: {}", result.gas_used);
            println!("Gas remaining: {}", result.gas_remaining);
        }
        Err(e) => {
            println!("Execution failed: {}", e);
        }
    }

    println!("\nEVM demonstration completed!");
    println!("\nThis is an improved implementation of the Ethereum Virtual Machine in Rust.");
    println!("It includes:");
    println!("- Core data types (Address, Uint256, Bytes, Hash)");
    println!("- Stack operations (push, pop, dup, swap)");
    println!("- Memory management");
    println!("- Storage operations");
    println!("- Gas metering");
    println!("- Enhanced opcode execution");
    println!("- SHA3 cryptographic operations");
    println!("- Bitwise operations (BYTE, SHL, SHR)");
    println!("- Improved error handling and validation");
    println!("- Debug and profiling capabilities");
    println!("\nTo extend this implementation further, you could add:");
    println!("- More opcodes (CALL, CREATE, DELEGATECALL, etc.)");
    println!("- Contract creation and calling");
    println!("- Event logging (LOG0-LOG4)");
    println!("- Block and transaction information");
    println!("- Advanced arithmetic (SDIV, SMOD, ADDMOD, MULMOD, SIGNEXTEND)");
    println!("- More sophisticated gas calculations");
    println!("- Network integration");
    println!("- State management");
}