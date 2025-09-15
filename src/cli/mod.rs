use clap::{Parser, Subcommand};
use crate::{
    types::{Address, Uint256, Bytes},
    executor::{Executor, ExecutionContext},
    validation::Validator,
    tracing::ExecutionTracer,
    advanced::{AdvancedEVM, GasOptimization, ContractAnalyzer},
};
use std::str::FromStr;
use num_bigint::BigUint;
use num_traits::Num;

/// Ethereum Virtual Machine in Rust - Command Line Interface
#[derive(Parser)]
#[command(name = "evm-rust")]
#[command(about = "A comprehensive EVM implementation in Rust")]
#[command(long_about = "EVM Rust is a complete implementation of the Ethereum Virtual Machine written in Rust. 

This CLI provides tools for:
• Executing EVM bytecode with full validation
• Detailed execution tracing and analysis
• Comprehensive gas metering and optimization
• Event logging and blockchain context simulation
• Interactive examples and testing

Examples:
  evm-rust execute --code '6002600301' --debug
  evm-rust examples --list
  evm-rust info --opcodes --gas-costs
  evm-rust execute --code '6002600301' --detailed-trace --export-trace trace.csv")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Execute EVM bytecode with full validation and tracing
    Execute {
        /// Hex-encoded bytecode to execute (e.g., "6002600301" for PUSH1 2 PUSH1 3 ADD)
        #[arg(short, long)]
        code: String,
        
        /// Gas limit for execution (21,000 - 30,000,000)
        #[arg(short, long, default_value = "1000000")]
        gas_limit: u64,
        
        /// Enable debug mode with detailed output
        #[arg(long)]
        debug: bool,
        
        /// Enable execution tracing
        #[arg(long)]
        trace: bool,
        
        /// Caller address (42-character hex string starting with 0x)
        #[arg(long, default_value = "0x0000000000000000000000000000000000000000")]
        caller: String,
        
        /// Contract address (42-character hex string starting with 0x)
        #[arg(long, default_value = "0x0000000000000000000000000000000000000000")]
        address: String,
        
        /// Call value in wei (decimal or hex with 0x prefix)
        #[arg(long, default_value = "0")]
        value: String,
        
        /// Input data (hex string, can start with 0x)
        #[arg(long, default_value = "")]
        input: String,
        
        /// Disable validation checks (not recommended for production)
        #[arg(long)]
        no_validate: bool,
        
        /// Enable detailed execution tracing
        #[arg(long)]
        detailed_trace: bool,
        
        /// Export execution trace to file (JSON/CSV format)
        #[arg(long)]
        export_trace: Option<String>,
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
        
        /// Show validation limits
        #[arg(long)]
        validation: bool,
    },
    
    /// Advanced EVM features
    Advanced {
        #[command(subcommand)]
        command: AdvancedCommands,
    },
}

#[derive(Subcommand)]
pub enum AdvancedCommands {
    /// Optimize bytecode for gas efficiency
    Optimize {
        /// Input bytecode file (hex)
        #[arg(short, long)]
        input: String,
        
        /// Output file for optimized bytecode
        #[arg(short, long)]
        output: Option<String>,
        
        /// Enable peephole optimization
        #[arg(long)]
        peephole: bool,
        
        /// Enable constant folding
        #[arg(long)]
        constant_folding: bool,
        
        /// Enable dead code elimination
        #[arg(long)]
        dead_code: bool,
    },
    
    /// Analyze contract bytecode
    Analyze {
        /// Bytecode to analyze (hex)
        #[arg(short, long)]
        code: String,
        
        /// Show detailed analysis
        #[arg(long)]
        detailed: bool,
    },
    
    /// Benchmark execution performance
    Benchmark {
        /// Bytecode to benchmark (hex)
        #[arg(short, long)]
        code: String,
        
        /// Number of iterations
        #[arg(short, long, default_value = "100")]
        iterations: u32,
        
        /// Gas limit for each execution
        #[arg(long, default_value = "1000000")]
        gas_limit: u64,
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
                input,
                no_validate,
                detailed_trace,
                export_trace
            } => {
                Self::execute_bytecode_static(code, gas_limit, debug, trace, caller, address, value, input, no_validate, detailed_trace, export_trace)
            }
            Commands::Examples { number, list } => {
                Self::run_examples_static(number, list)
            }
            Commands::Shell { gas_limit } => {
                Self::run_shell_static(gas_limit)
            }
            Commands::Info { opcodes, gas_costs, validation } => {
                Self::show_info_static(opcodes, gas_costs, validation)
            }
            Commands::Advanced { command } => {
                Self::handle_advanced_command(command)
            }
        }
    }
    
    /// Execute bytecode
    fn execute_bytecode_static(
        code: String,
        gas_limit: u64,
        debug: bool,
        _trace: bool,
        caller: String,
        address: String,
        value: String,
        input: String,
        no_validate: bool,
        detailed_trace: bool,
        export_trace: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 EVM Execution");
        println!("===============");
        
        // Create validator
        let validator = Validator::new();
        
        // Parse and validate hex code
        let code_bytes = if code.starts_with("0x") {
            hex::decode(&code[2..])?
        } else {
            hex::decode(&code)?
        };
        
        if !no_validate {
            println!("🔍 Validating bytecode...");
            validator.validate_bytecode(&code_bytes)?;
            println!("✅ Bytecode validation passed");
        }
        
        // Parse and validate addresses
        let caller_addr = if no_validate {
            Address::from_hex(&caller)?
        } else {
            validator.validate_address(&caller)?
        };
        
        let contract_addr = if no_validate {
            Address::from_hex(&address)?
        } else {
            validator.validate_address(&address)?
        };
        
        // Parse and validate value
        let call_value = if no_validate {
            if value.starts_with("0x") {
                let big_uint = BigUint::from_str_radix(&value[2..], 16)?;
                Uint256::new(big_uint)
            } else {
                let big_uint = BigUint::from_str(&value)?;
                Uint256::new(big_uint)
            }
        } else {
            validator.validate_value(&value)?
        };
        
        // Parse and validate input data
        let input_data = if no_validate {
            if input.is_empty() {
                Bytes::empty()
            } else if input.starts_with("0x") {
                Bytes::from(hex::decode(&input[2..])?)
            } else {
                Bytes::from(hex::decode(&input)?)
            }
        } else {
            validator.validate_input_data(&input)?
        };
        
        // Validate gas limit
        if !no_validate {
            validator.validate_gas_limit(gas_limit)?;
        }
        
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
        
        // Create tracer if tracing is enabled
        let tracer = if detailed_trace || export_trace.is_some() {
            Some(ExecutionTracer::new())
        } else {
            None
        };
        
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
        
        // Handle tracing
        if let Some(tracer) = tracer {
            let execution_trace = tracer.finalize(result.success, result.gas_used);
            
            if detailed_trace {
                println!("\n📈 Execution Trace:");
                println!("==================");
                println!("{}", execution_trace.summary());
                
                if !execution_trace.steps.is_empty() {
                    println!("\nStep-by-step execution:");
                    for (i, step) in execution_trace.steps.iter().enumerate() {
                        println!("  Step {}: {}", i + 1, step);
                    }
                }
            }
            
            // Export trace if requested
            if let Some(filename) = export_trace {
                match execution_trace.to_json() {
                    Ok(json_trace) => {
                        std::fs::write(&filename, json_trace)?;
                        println!("\n💾 Trace exported to: {}", filename);
                    }
                    Err(e) => {
                        println!("\n⚠️  JSON export failed: {}", e);
                        // Export as CSV instead
                        let csv_trace = execution_trace.to_csv();
                        let csv_filename = filename.replace(".json", ".csv");
                        std::fs::write(&csv_filename, csv_trace)?;
                        println!("💾 Trace exported as CSV to: {}", csv_filename);
                    }
                }
            }
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
    fn show_info_static(opcodes: bool, gas_costs: bool, validation: bool) -> Result<(), Box<dyn std::error::Error>> {
        if opcodes {
            Self::show_opcodes();
        }
        
        if gas_costs {
            Self::show_gas_costs();
        }
        
        if validation {
            Self::show_validation_info();
        }
        
        if !opcodes && !gas_costs && !validation {
            println!("ℹ️  EVM Information");
            println!("==================");
            println!("Use --opcodes to show opcode information");
            println!("Use --gas-costs to show gas cost information");
            println!("Use --validation to show validation limits");
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
    
    /// Show validation information
    fn show_validation_info() {
        let validator = Validator::new();
        let stats = validator.get_stats();
        println!("🛡️  Validation Limits:");
        println!("=====================");
        println!("{}", stats);
    }
    
    /// Handle advanced commands
    fn handle_advanced_command(command: AdvancedCommands) -> Result<(), Box<dyn std::error::Error>> {
        match command {
            AdvancedCommands::Optimize { input, output, peephole, constant_folding, dead_code } => {
                Self::optimize_bytecode(input, output, peephole, constant_folding, dead_code)
            }
            AdvancedCommands::Analyze { code, detailed } => {
                Self::analyze_contract(code, detailed)
            }
            AdvancedCommands::Benchmark { code, iterations, gas_limit } => {
                Self::benchmark_execution(code, iterations, gas_limit)
            }
        }
    }
    
    /// Optimize bytecode
    fn optimize_bytecode(
        input: String,
        output: Option<String>,
        peephole: bool,
        constant_folding: bool,
        dead_code: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔧 Bytecode Optimization");
        println!("========================");
        
        // Read input bytecode
        let bytecode = if input.starts_with("0x") {
            hex::decode(&input[2..])?
        } else {
            hex::decode(&input)?
        };
        
        println!("📥 Input bytecode: {} bytes", bytecode.len());
        println!("📥 Input hex: 0x{}", hex::encode(&bytecode));
        
        // Create optimizer with specified options
        let mut optimizer = GasOptimization::new();
        optimizer.peephole_optimization = peephole;
        optimizer.constant_folding = constant_folding;
        optimizer.dead_code_elimination = dead_code;
        
        // Optimize bytecode
        let optimized = optimizer.optimize(&bytecode)?;
        
        println!("📤 Optimized bytecode: {} bytes", optimized.len());
        println!("📤 Optimized hex: 0x{}", hex::encode(&optimized));
        println!("💾 Size reduction: {} bytes ({:.1}%)", 
            bytecode.len() - optimized.len(),
            ((bytecode.len() - optimized.len()) as f64 / bytecode.len() as f64) * 100.0
        );
        
        // Write output if specified
        if let Some(output_file) = output {
            std::fs::write(&output_file, hex::encode(&optimized))?;
            println!("💾 Optimized bytecode saved to: {}", output_file);
        }
        
        Ok(())
    }
    
    /// Analyze contract
    fn analyze_contract(code: String, detailed: bool) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 Contract Analysis");
        println!("===================");
        
        // Parse bytecode
        let bytecode = if code.starts_with("0x") {
            hex::decode(&code[2..])?
        } else {
            hex::decode(&code)?
        };
        
        // Create analyzer
        let analyzer = ContractAnalyzer::new();
        let analysis = analyzer.analyze(&bytecode);
        
        // Display analysis
        println!("{}", analysis);
        
        if detailed {
            println!("\n📊 Detailed Opcode Analysis:");
            println!("============================");
            let mut sorted_opcodes: Vec<_> = analysis.opcode_frequency.iter().collect();
            sorted_opcodes.sort_by(|a, b| b.1.cmp(a.1));
            
            for (opcode, count) in sorted_opcodes {
                println!("  {}: {} times", opcode, count);
            }
            
            if !analysis.function_selectors.is_empty() {
                println!("\n🔗 Function Selectors:");
                println!("=====================");
                for selector in analysis.function_selectors {
                    println!("  0x{}", hex::encode(selector));
                }
            }
        }
        
        Ok(())
    }
    
    /// Benchmark execution
    fn benchmark_execution(
        code: String,
        iterations: u32,
        gas_limit: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("⚡ Performance Benchmark");
        println!("========================");
        
        // Parse bytecode
        let bytecode = if code.starts_with("0x") {
            hex::decode(&code[2..])?
        } else {
            hex::decode(&code)?
        };
        
        println!("📊 Benchmarking {} iterations...", iterations);
        
        // Create advanced EVM
        let mut advanced_evm = AdvancedEVM::new();
        
        // Run benchmark
        let mut total_time = 0u64;
        let mut successful_executions = 0;
        
        for i in 0..iterations {
            let metrics = advanced_evm.monitor_execution(|| {
                let context = ExecutionContext::new(
                    Address::zero(),
                    Address::zero(),
                    Uint256::zero(),
                    Bytes::empty(),
                    Bytes::from(bytecode.clone()),
                    gas_limit,
                );
                
                let mut executor = Executor::new(context);
                executor.execute().unwrap_or_else(|_| crate::executor::ExecutionResult {
                    success: false,
                    gas_used: 0,
                    gas_remaining: gas_limit,
                    return_data: Bytes::empty(),
                    logs: vec![],
                })
            });
            
            total_time += metrics.execution_time_us;
            if metrics.success {
                successful_executions += 1;
            }
            
            if (i + 1) % 10 == 0 {
                print!(".");
                std::io::Write::flush(&mut std::io::stdout())?;
            }
        }
        
        println!("\n\n📈 Benchmark Results:");
        println!("====================");
        println!("  Total Iterations: {}", iterations);
        println!("  Successful: {}", successful_executions);
        println!("  Failed: {}", iterations - successful_executions);
        println!("  Average Time: {:.2}μs", total_time as f64 / iterations as f64);
        println!("  Total Time: {:.2}ms", total_time as f64 / 1000.0);
        
        let stats = advanced_evm.performance_monitor.get_stats();
        println!("\n{}", stats);
        
        Ok(())
    }
}
