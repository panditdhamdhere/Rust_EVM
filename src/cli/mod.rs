use clap::{Parser, Subcommand};
use crate::{
    types::{Address, Uint256, Bytes},
    executor::{Executor, ExecutionContext},
};
use std::str::FromStr;
use num_bigint::BigUint;
use num_traits::Num;

/// Ethereum Virtual Machine in Rust - Command Line Interface
#[derive(Parser)]
#[command(name = "evm-rust")]
#[command(about = "A comprehensive EVM implementation in Rust")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Execute EVM bytecode
    Execute {
        /// Hex-encoded bytecode to execute
        #[arg(short, long)]
        code: String,
        
        /// Gas limit for execution
        #[arg(short, long, default_value = "1000000")]
        gas_limit: u64,
        
        /// Enable debug mode with detailed output
        #[arg(long)]
        debug: bool,
        
        /// Enable execution tracing
        #[arg(long)]
        trace: bool,
        
        /// Caller address (hex)
        #[arg(long, default_value = "0x0000000000000000000000000000000000000000")]
        caller: String,
        
        /// Contract address (hex)
        #[arg(long, default_value = "0x0000000000000000000000000000000000000000")]
        address: String,
        
        /// Call value in wei
        #[arg(long, default_value = "0")]
        value: String,
        
        /// Input data (hex)
        #[arg(long, default_value = "")]
        input: String,
    },
    
    /// Run predefined examples
    Examples {
        /// Example to run (1-10)
        #[arg(short, long)]
        number: Option<u8>,
        
        /// List all available examples
        #[arg(long)]
        list: bool,
    },
    
    /// Interactive EVM shell
    Shell {
        /// Gas limit for shell execution
        #[arg(short, long, default_value = "1000000")]
        gas_limit: u64,
    },
    
    /// Show EVM information
    Info {
        /// Show opcode information
        #[arg(long)]
        opcodes: bool,
        
        /// Show gas costs
        #[arg(long)]
        gas_costs: bool,
    },
}

impl Cli {
    /// Parse command line arguments
    pub fn parse_args() -> Self {
        Cli::parse()
    }
    
    /// Run the CLI
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            Commands::Execute { 
                code, 
                gas_limit, 
                debug, 
                trace, 
                caller, 
                address, 
                value, 
                input 
            } => {
                Self::execute_bytecode_static(code, gas_limit, debug, trace, caller, address, value, input)
            }
            Commands::Examples { number, list } => {
                Self::run_examples_static(number, list)
            }
            Commands::Shell { gas_limit } => {
                Self::run_shell_static(gas_limit)
            }
            Commands::Info { opcodes, gas_costs } => {
                Self::show_info_static(opcodes, gas_costs)
            }
        }
    }
    
    /// Execute bytecode
    fn execute_bytecode_static(
        code: String,
        gas_limit: u64,
        debug: bool,
        trace: bool,
        caller: String,
        address: String,
        value: String,
        input: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 EVM Execution");
        println!("===============");
        
        // Parse hex code
        let code_bytes = if code.starts_with("0x") {
            hex::decode(&code[2..])?
        } else {
            hex::decode(&code)?
        };
        
        // Parse addresses
        let caller_addr = Address::from_hex(&caller)?;
        let contract_addr = Address::from_hex(&address)?;
        
        // Parse value
        let call_value = if value.starts_with("0x") {
            let big_uint = BigUint::from_str_radix(&value[2..], 16)?;
            Uint256::new(big_uint)
        } else {
            let big_uint = BigUint::from_str(&value)?;
            Uint256::new(big_uint)
        };
        
        // Parse input data
        let input_data = if input.is_empty() {
            Bytes::empty()
        } else if input.starts_with("0x") {
            Bytes::from(hex::decode(&input[2..])?)
        } else {
            Bytes::from(hex::decode(&input)?)
        };
        
        if debug {
            println!("📋 Execution Context:");
            println!("  Contract: {}", contract_addr);
            println!("  Caller: {}", caller_addr);
            println!("  Value: {} wei", call_value);
            println!("  Gas Limit: {}", gas_limit);
            println!("  Code Size: {} bytes", code_bytes.len());
            println!();
        }
        
        // Create execution context
        let context = ExecutionContext::new(
            contract_addr,
            caller_addr,
            call_value,
            input_data,
            Bytes::from(code_bytes),
            gas_limit,
        );
        
        // Execute
        let mut executor = Executor::new(context);
        let result = executor.execute()?;
        
        // Display results
        println!("📊 Execution Results:");
        println!("  Success: {}", result.success);
        println!("  Gas Used: {}", result.gas_used);
        println!("  Gas Remaining: {}", result.gas_remaining);
        
        if !result.return_data.is_empty() {
            println!("  Return Data: 0x{}", hex::encode(result.return_data.as_slice()));
        }
        
        if !result.logs.is_empty() {
            println!("  Event Logs: {}", result.logs.len());
            for (i, log) in result.logs.iter().enumerate() {
                println!("    Log {}: {}", i, log);
            }
        }
        
        if trace {
            println!("\n🔍 Execution Trace:");
            // TODO: Implement execution tracing
            println!("  (Tracing not yet implemented)");
        }
        
        Ok(())
    }
    
    /// Run examples
    fn run_examples_static(number: Option<u8>, list: bool) -> Result<(), Box<dyn std::error::Error>> {
        if list {
            Self::list_examples();
            return Ok(());
        }
        
        if let Some(num) = number {
            Self::run_single_example(num)?;
        } else {
            Self::run_all_examples()?;
        }
        
        Ok(())
    }
    
    /// List all examples
    fn list_examples() {
        println!("📚 Available Examples:");
        println!("  1. Simple Arithmetic (ADD)");
        println!("  2. Stack Operations (DUP)");
        println!("  3. Memory Operations (MSTORE8/MLOAD)");
        println!("  4. Comparison Operations (GT)");
        println!("  5. Bitwise Operations (AND)");
        println!("  6. SHA3 Cryptographic Operation");
        println!("  7. Byte Operation (BYTE)");
        println!("  8. Event Logging (LOG0)");
        println!("  9. Block Information (TIMESTAMP, NUMBER, CHAINID)");
        println!("  10. Advanced Arithmetic (ADDMOD)");
        println!();
        println!("Usage: evm-rust examples --number <1-10>");
    }
    
    /// Run a single example
    fn run_single_example(number: u8) -> Result<(), Box<dyn std::error::Error>> {
        match number {
            1 => Self::run_example_1(),
            2 => Self::run_example_2(),
            3 => Self::run_example_3(),
            4 => Self::run_example_4(),
            5 => Self::run_example_5(),
            6 => Self::run_example_6(),
            7 => Self::run_example_7(),
            8 => Self::run_example_8(),
            9 => Self::run_example_9(),
            10 => Self::run_example_10(),
            _ => {
                println!("❌ Invalid example number. Use --list to see available examples.");
                Ok(())
            }
        }
    }
    
    /// Run all examples
    fn run_all_examples() -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Running All Examples");
        println!("======================");
        
        for i in 1..=10 {
            println!("\n--- Example {} ---", i);
            Self::run_single_example(i)?;
        }
        
        Ok(())
    }
    
    /// Example 1: Simple Arithmetic
    fn run_example_1() -> Result<(), Box<dyn std::error::Error>> {
        println!("1. Simple Arithmetic Example:");
        println!("Code: PUSH1 0x02 PUSH1 0x03 ADD STOP");
        
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
        let result = executor.execute()?;
        
        println!("  Result: Success={}, Gas={}", result.success, result.gas_used);
        Ok(())
    }
    
    /// Example 2: Stack Operations
    fn run_example_2() -> Result<(), Box<dyn std::error::Error>> {
        println!("2. Stack Operations Example:");
        println!("Code: PUSH1 0x01 PUSH1 0x02 DUP1 ADD STOP");
        
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
        let result = executor.execute()?;
        
        println!("  Result: Success={}, Gas={}", result.success, result.gas_used);
        Ok(())
    }
    
    /// Example 3: Memory Operations
    fn run_example_3() -> Result<(), Box<dyn std::error::Error>> {
        println!("3. Memory Operations Example:");
        println!("Code: PUSH1 0x00 PUSH1 0x42 MSTORE8 PUSH1 0x00 MLOAD STOP");
        
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
        let result = executor.execute()?;
        
        println!("  Result: Success={}, Gas={}", result.success, result.gas_used);
        Ok(())
    }
    
    /// Example 4: Comparison Operations
    fn run_example_4() -> Result<(), Box<dyn std::error::Error>> {
        println!("4. Comparison Operations Example:");
        println!("Code: PUSH1 0x05 PUSH1 0x03 GT STOP");
        
        let code = Bytes::from(vec![0x60, 0x05, 0x60, 0x03, 0x11, 0x00]);
        let context = ExecutionContext::new(
            Address::zero(),
            Address::zero(),
            Uint256::zero(),
            Bytes::empty(),
            code,
            1000,
        );
        
        let mut executor = Executor::new(context);
        let result = executor.execute()?;
        
        println!("  Result: Success={}, Gas={}", result.success, result.gas_used);
        Ok(())
    }
    
    /// Example 5: Bitwise Operations
    fn run_example_5() -> Result<(), Box<dyn std::error::Error>> {
        println!("5. Bitwise Operations Example:");
        println!("Code: PUSH1 0x0F PUSH1 0x33 AND STOP");
        
        let code = Bytes::from(vec![0x60, 0x0F, 0x60, 0x33, 0x16, 0x00]);
        let context = ExecutionContext::new(
            Address::zero(),
            Address::zero(),
            Uint256::zero(),
            Bytes::empty(),
            code,
            1000,
        );
        
        let mut executor = Executor::new(context);
        let result = executor.execute()?;
        
        println!("  Result: Success={}, Gas={}", result.success, result.gas_used);
        Ok(())
    }
    
    /// Example 6: SHA3 Operation
    fn run_example_6() -> Result<(), Box<dyn std::error::Error>> {
        println!("6. SHA3 Operation Example:");
        println!("Code: PUSH1 0x00 PUSH1 0x20 MSTORE PUSH1 0x00 PUSH1 0x20 SHA3 STOP");
        
        let code = Bytes::from(vec![
            0x60, 0x00, 0x60, 0x20, 0x52, 0x60, 0x00, 0x60, 0x20, 0x20, 0x00
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
        let result = executor.execute()?;
        
        println!("  Result: Success={}, Gas={}", result.success, result.gas_used);
        Ok(())
    }
    
    /// Example 7: Byte Operation
    fn run_example_7() -> Result<(), Box<dyn std::error::Error>> {
        println!("7. Byte Operation Example:");
        println!("Code: PUSH4 0x12345678 PUSH1 0x01 BYTE POP STOP");
        
        let code = Bytes::from(vec![
            0x63, 0x12, 0x34, 0x56, 0x78, 0x60, 0x01, 0x1a, 0x50, 0x00
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
        let result = executor.execute()?;
        
        println!("  Result: Success={}, Gas={}", result.success, result.gas_used);
        Ok(())
    }
    
    /// Example 8: Event Logging
    fn run_example_8() -> Result<(), Box<dyn std::error::Error>> {
        println!("8. Event Logging Example:");
        println!("Code: PUSH1 0x00 PUSH1 0x20 MSTORE PUSH1 0x00 PUSH1 0x20 LOG0 STOP");
        
        let code = Bytes::from(vec![
            0x60, 0x00, 0x60, 0x20, 0x52, 0x60, 0x00, 0x60, 0x20, 0xa0, 0x00
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
        let result = executor.execute()?;
        
        println!("  Result: Success={}, Gas={}, Logs={}", 
            result.success, result.gas_used, result.logs.len());
        Ok(())
    }
    
    /// Example 9: Block Information
    fn run_example_9() -> Result<(), Box<dyn std::error::Error>> {
        println!("9. Block Information Example:");
        println!("Code: TIMESTAMP NUMBER CHAINID STOP");
        
        let code = Bytes::from(vec![0x42, 0x43, 0x46, 0x00]);
        let context = ExecutionContext::new(
            Address::zero(),
            Address::zero(),
            Uint256::zero(),
            Bytes::empty(),
            code,
            1000,
        );
        
        let mut executor = Executor::new(context);
        let result = executor.execute()?;
        
        println!("  Result: Success={}, Gas={}", result.success, result.gas_used);
        Ok(())
    }
    
    /// Example 10: Advanced Arithmetic
    fn run_example_10() -> Result<(), Box<dyn std::error::Error>> {
        println!("10. Advanced Arithmetic Example:");
        println!("Code: PUSH1 0x05 PUSH1 0x03 PUSH1 0x02 ADDMOD STOP");
        
        let code = Bytes::from(vec![
            0x60, 0x05, 0x60, 0x03, 0x60, 0x02, 0x08, 0x00
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
        let result = executor.execute()?;
        
        println!("  Result: Success={}, Gas={}", result.success, result.gas_used);
        Ok(())
    }
    
    /// Run interactive shell
    fn run_shell_static(_gas_limit: u64) -> Result<(), Box<dyn std::error::Error>> {
        println!("🐚 EVM Interactive Shell");
        println!("=======================");
        println!("Type 'help' for commands, 'exit' to quit");
        println!();
        
        // TODO: Implement interactive shell
        println!("Interactive shell not yet implemented.");
        println!("Use 'evm-rust execute --code <hex>' to run bytecode.");
        
        Ok(())
    }
    
    /// Show EVM information
    fn show_info_static(opcodes: bool, gas_costs: bool) -> Result<(), Box<dyn std::error::Error>> {
        if opcodes {
            Self::show_opcodes();
        }
        
        if gas_costs {
            Self::show_gas_costs();
        }
        
        if !opcodes && !gas_costs {
            println!("ℹ️  EVM Information");
            println!("==================");
            println!("Use --opcodes to show opcode information");
            println!("Use --gas-costs to show gas cost information");
        }
        
        Ok(())
    }
    
    /// Show opcode information
    fn show_opcodes() {
        println!("📋 Supported Opcodes:");
        println!("====================");
        println!("Arithmetic: ADD, SUB, MUL, DIV, MOD, EXP, SDIV, SMOD, ADDMOD, MULMOD, SIGNEXTEND");
        println!("Comparison: LT, GT, SLT, SGT, EQ, ISZERO");
        println!("Bitwise: AND, OR, XOR, NOT, BYTE, SHL, SHR");
        println!("Cryptographic: SHA3");
        println!("Stack: PUSH1-32, POP, DUP1-16, SWAP1-16");
        println!("Memory: MLOAD, MSTORE, MSTORE8, MSIZE");
        println!("Storage: SLOAD, SSTORE");
        println!("Environmental: ADDRESS, CALLER, CALLVALUE, BALANCE, etc.");
        println!("Block Info: TIMESTAMP, NUMBER, CHAINID, COINBASE, etc.");
        println!("Logging: LOG0, LOG1, LOG2, LOG3, LOG4");
        println!("Control Flow: JUMP, JUMPI, PC, JUMPDEST");
        println!("System: STOP, RETURN, REVERT");
    }
    
    /// Show gas costs
    fn show_gas_costs() {
        println!("⛽ Gas Costs:");
        println!("============");
        println!("Base operations: 2-3 gas");
        println!("Arithmetic: 3-5 gas");
        println!("Memory operations: 3 gas + expansion cost");
        println!("Storage operations: 100-20000 gas (dynamic)");
        println!("SHA3: 30 gas + 6 gas per word");
        println!("Logging: 375-1875 gas (depending on topics)");
        println!("Block info: 2-20 gas");
    }
}
