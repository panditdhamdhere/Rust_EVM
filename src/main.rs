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
    
    println!("\nEVM demonstration completed!");
    println!("\nThis is a basic implementation of the Ethereum Virtual Machine in Rust.");
    println!("It includes:");
    println!("- Core data types (Address, Uint256, Bytes, Hash)");
    println!("- Stack operations (push, pop, dup, swap)");
    println!("- Memory management");
    println!("- Storage operations");
    println!("- Gas metering");
    println!("- Basic opcode execution");
    println!("\nTo extend this implementation, you could add:");
    println!("- More opcodes (SHA3, CALL, CREATE, etc.)");
    println!("- Cryptographic operations");
    println!("- Contract creation and calling");
    println!("- Event logging");
    println!("- More sophisticated gas calculations");
}