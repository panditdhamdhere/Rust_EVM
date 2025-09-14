use crate::{
    types::Uint256,
    stack::Stack,
    memory::Memory,
    gas::GasMeter,
    opcodes::Opcode,
};
use std::fmt;

/// Debug information for EVM execution
#[derive(Debug, Clone)]
pub struct DebugInfo {
    /// Program counter
    pub pc: usize,
    /// Current opcode
    pub opcode: Opcode,
    /// Stack state
    pub stack: Vec<Uint256>,
    /// Memory size
    pub memory_size: usize,
    /// Gas remaining
    pub gas_remaining: u64,
    /// Gas used
    pub gas_used: u64,
}

impl DebugInfo {
    /// Create new debug info
    pub fn new(
        pc: usize,
        opcode: Opcode,
        stack: &Stack,
        memory: &Memory,
        gas_meter: &GasMeter,
    ) -> Self {
        DebugInfo {
            pc,
            opcode,
            stack: stack.items().to_vec(),
            memory_size: memory.size(),
            gas_remaining: gas_meter.available(),
            gas_used: gas_meter.used(),
        }
    }
}

impl fmt::Display for DebugInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PC: 0x{:04x} | Opcode: {:?}", self.pc, self.opcode)?;
        writeln!(f, "Stack: [{}]", 
            self.stack.iter()
                .map(|x| format!("0x{}", x))
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        writeln!(f, "Memory: {} bytes | Gas: {} remaining ({} used)", 
            self.memory_size, self.gas_remaining, self.gas_used)?;
        Ok(())
    }
}

/// Debugger for EVM execution
pub struct Debugger {
    /// Enable debug mode
    pub enabled: bool,
    /// Step-by-step execution
    pub step_mode: bool,
    /// Breakpoints
    pub breakpoints: Vec<usize>,
    /// Execution trace
    pub trace: Vec<DebugInfo>,
}

impl Debugger {
    /// Create a new debugger
    pub fn new() -> Self {
        Debugger {
            enabled: false,
            step_mode: false,
            breakpoints: Vec::new(),
            trace: Vec::new(),
        }
    }

    /// Enable debug mode
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable debug mode
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Add a breakpoint
    pub fn add_breakpoint(&mut self, pc: usize) {
        if !self.breakpoints.contains(&pc) {
            self.breakpoints.push(pc);
        }
    }

    /// Remove a breakpoint
    pub fn remove_breakpoint(&mut self, pc: usize) {
        self.breakpoints.retain(|&x| x != pc);
    }

    /// Check if execution should break
    pub fn should_break(&self, pc: usize) -> bool {
        self.breakpoints.contains(&pc)
    }

    /// Record execution step
    pub fn record_step(&mut self, info: DebugInfo) {
        if self.enabled {
            self.trace.push(info);
        }
    }

    /// Get execution trace
    pub fn get_trace(&self) -> &[DebugInfo] {
        &self.trace
    }

    /// Clear trace
    pub fn clear_trace(&mut self) {
        self.trace.clear();
    }

    /// Print current state
    pub fn print_state(&self, info: &DebugInfo) {
        if self.enabled {
            println!("{}", info);
        }
    }
}

impl Default for Debugger {
    fn default() -> Self {
        Debugger::new()
    }
}

/// Gas usage analyzer
pub struct GasAnalyzer {
    /// Gas usage per opcode
    pub opcode_gas: std::collections::HashMap<Opcode, Vec<u64>>,
    /// Total gas usage
    pub total_gas: u64,
}

impl GasAnalyzer {
    /// Create a new gas analyzer
    pub fn new() -> Self {
        GasAnalyzer {
            opcode_gas: std::collections::HashMap::new(),
            total_gas: 0,
        }
    }

    /// Record gas usage for an opcode
    pub fn record_gas_usage(&mut self, opcode: Opcode, gas_used: u64) {
        self.opcode_gas.entry(opcode).or_insert_with(Vec::new).push(gas_used);
        self.total_gas += gas_used;
    }

    /// Get gas usage statistics
    pub fn get_stats(&self) -> GasStats {
        let mut opcode_stats = std::collections::HashMap::new();
        
        for (opcode, gas_usage) in &self.opcode_gas {
            let count = gas_usage.len();
            let total = gas_usage.iter().sum::<u64>();
            let average = if count > 0 { total / count as u64 } else { 0 };
            let min = gas_usage.iter().min().copied().unwrap_or(0);
            let max = gas_usage.iter().max().copied().unwrap_or(0);
            
            opcode_stats.insert(*opcode, OpcodeGasStats {
                count,
                total,
                average,
                min,
                max,
            });
        }
        
        GasStats {
            total_gas: self.total_gas,
            opcode_stats,
        }
    }
}

/// Gas usage statistics for an opcode
#[derive(Debug, Clone)]
pub struct OpcodeGasStats {
    /// Number of times the opcode was executed
    pub count: usize,
    /// Total gas used
    pub total: u64,
    /// Average gas per execution
    pub average: u64,
    /// Minimum gas used
    pub min: u64,
    /// Maximum gas used
    pub max: u64,
}

/// Overall gas statistics
#[derive(Debug, Clone)]
pub struct GasStats {
    /// Total gas used
    pub total_gas: u64,
    /// Statistics per opcode
    pub opcode_stats: std::collections::HashMap<Opcode, OpcodeGasStats>,
}

impl fmt::Display for GasStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Total Gas Used: {}", self.total_gas)?;
        writeln!(f, "Opcode Statistics:")?;
        
        let mut sorted_stats: Vec<_> = self.opcode_stats.iter().collect();
        sorted_stats.sort_by(|a, b| b.1.total.cmp(&a.1.total));
        
        for (opcode, stats) in sorted_stats {
            writeln!(f, "  {:?}: {} executions, {} total gas, {} avg gas", 
                opcode, stats.count, stats.total, stats.average)?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_info_creation() {
        let stack = Stack::new();
        let memory = Memory::new();
        let gas_meter = GasMeter::new(1000);
        
        let info = DebugInfo::new(0, Opcode::Add, &stack, &memory, &gas_meter);
        
        assert_eq!(info.pc, 0);
        assert_eq!(info.opcode, Opcode::Add);
        assert_eq!(info.gas_remaining, 1000);
        assert_eq!(info.gas_used, 0);
    }

    #[test]
    fn test_debugger_breakpoints() {
        let mut debugger = Debugger::new();
        
        debugger.add_breakpoint(100);
        debugger.add_breakpoint(200);
        
        assert!(debugger.should_break(100));
        assert!(debugger.should_break(200));
        assert!(!debugger.should_break(150));
        
        debugger.remove_breakpoint(100);
        assert!(!debugger.should_break(100));
        assert!(debugger.should_break(200));
    }

    #[test]
    fn test_gas_analyzer() {
        let mut analyzer = GasAnalyzer::new();
        
        analyzer.record_gas_usage(Opcode::Add, 3);
        analyzer.record_gas_usage(Opcode::Add, 3);
        analyzer.record_gas_usage(Opcode::Mul, 5);
        
        let stats = analyzer.get_stats();
        assert_eq!(stats.total_gas, 11);
        
        let add_stats = stats.opcode_stats.get(&Opcode::Add).unwrap();
        assert_eq!(add_stats.count, 2);
        assert_eq!(add_stats.total, 6);
        assert_eq!(add_stats.average, 3);
    }
}
