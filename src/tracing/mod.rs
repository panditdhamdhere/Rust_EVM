use crate::{
    types::Uint256,
    opcodes::Opcode,
};
use std::collections::HashMap;
use std::fmt;

/// Represents a single step in EVM execution
#[derive(Debug, Clone)]
pub struct ExecutionStep {
    /// Program counter (instruction pointer)
    pub pc: usize,
    /// Opcode being executed
    pub opcode: Opcode,
    /// Stack state before execution
    pub stack_before: Vec<Uint256>,
    /// Stack state after execution
    pub stack_after: Vec<Uint256>,
    /// Memory state (only changed regions)
    pub memory_changes: HashMap<usize, u8>,
    /// Storage changes (key -> (old_value, new_value))
    pub storage_changes: HashMap<Uint256, (Uint256, Uint256)>,
    /// Gas consumed in this step
    pub gas_consumed: u64,
    /// Gas remaining after this step
    pub gas_remaining: u64,
    /// Depth of the call stack
    pub depth: usize,
    /// Error if this step failed
    pub error: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ExecutionStep {
    /// Create a new execution step
    pub fn new(
        pc: usize,
        opcode: Opcode,
        stack_before: Vec<Uint256>,
        stack_after: Vec<Uint256>,
        gas_consumed: u64,
        gas_remaining: u64,
        depth: usize,
    ) -> Self {
        ExecutionStep {
            pc,
            opcode,
            stack_before,
            stack_after,
            memory_changes: HashMap::new(),
            storage_changes: HashMap::new(),
            gas_consumed,
            gas_remaining,
            depth,
            error: None,
            metadata: HashMap::new(),
        }
    }

    /// Add memory change
    pub fn add_memory_change(&mut self, offset: usize, value: u8) {
        self.memory_changes.insert(offset, value);
    }

    /// Add storage change
    pub fn add_storage_change(&mut self, key: Uint256, old_value: Uint256, new_value: Uint256) {
        self.storage_changes.insert(key, (old_value, new_value));
    }

    /// Set error
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get stack difference
    pub fn stack_difference(&self) -> i32 {
        self.stack_after.len() as i32 - self.stack_before.len() as i32
    }

    /// Check if this step modified storage
    pub fn modified_storage(&self) -> bool {
        !self.storage_changes.is_empty()
    }

    /// Check if this step modified memory
    pub fn modified_memory(&self) -> bool {
        !self.memory_changes.is_empty()
    }
}

impl fmt::Display for ExecutionStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PC:{} {} | Gas:{}→{} | Stack:{}→{}", 
            self.pc, 
            self.opcode, 
            self.gas_consumed, 
            self.gas_remaining,
            self.stack_before.len(),
            self.stack_after.len()
        )?;
        
        if !self.memory_changes.is_empty() {
            write!(f, " | Memory:{} changes", self.memory_changes.len())?;
        }
        
        if !self.storage_changes.is_empty() {
            write!(f, " | Storage:{} changes", self.storage_changes.len())?;
        }
        
        if let Some(ref error) = self.error {
            write!(f, " | ERROR: {}", error)?;
        }
        
        Ok(())
    }
}

/// Execution trace containing all steps
#[derive(Debug, Clone)]
pub struct ExecutionTrace {
    /// All execution steps
    pub steps: Vec<ExecutionStep>,
    /// Final execution result
    pub success: bool,
    /// Total gas consumed
    pub total_gas_consumed: u64,
    /// Total execution time (microseconds)
    pub execution_time_us: u64,
    /// Number of opcodes executed
    pub opcode_count: usize,
    /// Gas efficiency (opcodes per gas unit)
    pub gas_efficiency: f64,
    /// Memory usage statistics
    pub memory_stats: MemoryStats,
    /// Storage access statistics
    pub storage_stats: StorageStats,
    /// Opcode frequency analysis
    pub opcode_frequency: HashMap<Opcode, usize>,
}

impl ExecutionTrace {
    /// Create a new execution trace
    pub fn new() -> Self {
        ExecutionTrace {
            steps: Vec::new(),
            success: false,
            total_gas_consumed: 0,
            execution_time_us: 0,
            opcode_count: 0,
            gas_efficiency: 0.0,
            memory_stats: MemoryStats::new(),
            storage_stats: StorageStats::new(),
            opcode_frequency: HashMap::new(),
        }
    }

    /// Add a step to the trace
    pub fn add_step(&mut self, step: ExecutionStep) {
        // Update opcode frequency
        *self.opcode_frequency.entry(step.opcode).or_insert(0) += 1;
        
        // Update memory stats
        self.memory_stats.update(&step);
        
        // Update storage stats
        self.storage_stats.update(&step);
        
        self.steps.push(step);
        self.opcode_count += 1;
    }

    /// Finalize the trace with execution results
    pub fn finalize(&mut self, success: bool, total_gas_consumed: u64, execution_time_us: u64) {
        self.success = success;
        self.total_gas_consumed = total_gas_consumed;
        self.execution_time_us = execution_time_us;
        
        if total_gas_consumed > 0 {
            self.gas_efficiency = self.opcode_count as f64 / total_gas_consumed as f64;
        }
        
        self.memory_stats.finalize();
        self.storage_stats.finalize();
    }

    /// Get steps that modified storage
    pub fn storage_modifying_steps(&self) -> Vec<&ExecutionStep> {
        self.steps.iter().filter(|step| step.modified_storage()).collect()
    }

    /// Get steps that modified memory
    pub fn memory_modifying_steps(&self) -> Vec<&ExecutionStep> {
        self.steps.iter().filter(|step| step.modified_memory()).collect()
    }

    /// Get steps that failed
    pub fn failed_steps(&self) -> Vec<&ExecutionStep> {
        self.steps.iter().filter(|step| step.error.is_some()).collect()
    }

    /// Get gas consumption by opcode
    pub fn gas_by_opcode(&self) -> HashMap<Opcode, u64> {
        let mut gas_by_opcode = HashMap::new();
        
        for step in &self.steps {
            *gas_by_opcode.entry(step.opcode).or_insert(0) += step.gas_consumed;
        }
        
        gas_by_opcode
    }

    /// Get execution summary
    pub fn summary(&self) -> ExecutionSummary {
        ExecutionSummary {
            total_steps: self.steps.len(),
            success: self.success,
            total_gas_consumed: self.total_gas_consumed,
            execution_time_us: self.execution_time_us,
            gas_efficiency: self.gas_efficiency,
            memory_peak: self.memory_stats.peak_size,
            storage_accesses: self.storage_stats.total_accesses,
            unique_opcodes: self.opcode_frequency.len(),
            most_frequent_opcode: self.most_frequent_opcode(),
        }
    }

    /// Get the most frequently used opcode
    fn most_frequent_opcode(&self) -> Option<(Opcode, usize)> {
        self.opcode_frequency.iter()
            .max_by_key(|(_, count)| *count)
            .map(|(opcode, count)| (*opcode, *count))
    }

    /// Export trace to JSON (placeholder - requires Serialize)
    pub fn to_json(&self) -> Result<String, String> {
        Err("JSON export not implemented - requires Serialize trait".to_string())
    }

    /// Export trace to CSV format
    pub fn to_csv(&self) -> String {
        let mut csv = String::new();
        csv.push_str("PC,Opcode,Gas_Consumed,Gas_Remaining,Stack_Before,Stack_After,Memory_Changes,Storage_Changes,Error\n");
        
        for step in &self.steps {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                step.pc,
                step.opcode,
                step.gas_consumed,
                step.gas_remaining,
                step.stack_before.len(),
                step.stack_after.len(),
                step.memory_changes.len(),
                step.storage_changes.len(),
                step.error.as_ref().unwrap_or(&"".to_string())
            ));
        }
        
        csv
    }
}

/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub peak_size: usize,
    pub total_allocations: usize,
    pub total_writes: usize,
    pub allocation_points: Vec<usize>,
}

impl MemoryStats {
    pub fn new() -> Self {
        MemoryStats {
            peak_size: 0,
            total_allocations: 0,
            total_writes: 0,
            allocation_points: Vec::new(),
        }
    }

    pub fn update(&mut self, step: &ExecutionStep) {
        self.total_writes += step.memory_changes.len();
        
        // Track memory allocations (simplified)
        if step.modified_memory() {
            self.total_allocations += 1;
            self.allocation_points.push(step.pc);
        }
    }

    pub fn finalize(&mut self) {
        // Calculate peak memory usage
        // This is a simplified calculation
        self.peak_size = self.total_writes * 32; // Assume 32-byte words
    }
}

/// Storage access statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub total_accesses: usize,
    pub unique_keys: usize,
    pub read_count: usize,
    pub write_count: usize,
    pub accessed_keys: Vec<Uint256>,
}

impl StorageStats {
    pub fn new() -> Self {
        StorageStats {
            total_accesses: 0,
            unique_keys: 0,
            read_count: 0,
            write_count: 0,
            accessed_keys: Vec::new(),
        }
    }

    pub fn update(&mut self, step: &ExecutionStep) {
        for (key, (old_value, new_value)) in &step.storage_changes {
            self.total_accesses += 1;
            
            if !self.accessed_keys.contains(key) {
                self.accessed_keys.push(key.clone());
                self.unique_keys += 1;
            }
            
            if *old_value != *new_value {
                self.write_count += 1;
            } else {
                self.read_count += 1;
            }
        }
    }

    pub fn finalize(&mut self) {
        // Finalize storage statistics
    }
}

/// Execution summary
#[derive(Debug, Clone)]
pub struct ExecutionSummary {
    pub total_steps: usize,
    pub success: bool,
    pub total_gas_consumed: u64,
    pub execution_time_us: u64,
    pub gas_efficiency: f64,
    pub memory_peak: usize,
    pub storage_accesses: usize,
    pub unique_opcodes: usize,
    pub most_frequent_opcode: Option<(Opcode, usize)>,
}

impl fmt::Display for ExecutionSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Execution Summary:\n")?;
        write!(f, "  Steps: {}\n", self.total_steps)?;
        write!(f, "  Success: {}\n", self.success)?;
        write!(f, "  Gas Consumed: {}\n", self.total_gas_consumed)?;
        write!(f, "  Execution Time: {}μs\n", self.execution_time_us)?;
        write!(f, "  Gas Efficiency: {:.4} opcodes/gas\n", self.gas_efficiency)?;
        write!(f, "  Peak Memory: {} bytes\n", self.memory_peak)?;
        write!(f, "  Storage Accesses: {}\n", self.storage_accesses)?;
        write!(f, "  Unique Opcodes: {}\n", self.unique_opcodes)?;
        
        if let Some((opcode, count)) = self.most_frequent_opcode {
            write!(f, "  Most Frequent: {} ({} times)", opcode, count)?;
        }
        
        Ok(())
    }
}

/// Execution tracer for collecting detailed execution information
pub struct ExecutionTracer {
    trace: ExecutionTrace,
    start_time: std::time::Instant,
    current_step: Option<ExecutionStep>,
}

impl ExecutionTracer {
    /// Create a new execution tracer
    pub fn new() -> Self {
        ExecutionTracer {
            trace: ExecutionTrace::new(),
            start_time: std::time::Instant::now(),
            current_step: None,
        }
    }

    /// Start tracing a new step
    pub fn start_step(&mut self, pc: usize, opcode: Opcode, stack_before: Vec<Uint256>, gas_remaining: u64, depth: usize) {
        self.current_step = Some(ExecutionStep::new(
            pc,
            opcode,
            stack_before,
            Vec::new(), // Will be set in end_step
            0, // Will be calculated in end_step
            gas_remaining,
            depth,
        ));
    }

    /// End the current step
    pub fn end_step(&mut self, stack_after: Vec<Uint256>, gas_consumed: u64, gas_remaining: u64) {
        if let Some(mut step) = self.current_step.take() {
            step.stack_after = stack_after;
            step.gas_consumed = gas_consumed;
            step.gas_remaining = gas_remaining;
            self.trace.add_step(step);
        }
    }

    /// Record an error in the current step
    pub fn record_error(&mut self, error: String) {
        if let Some(ref mut step) = self.current_step {
            step.set_error(error);
        }
    }

    /// Record memory change
    pub fn record_memory_change(&mut self, offset: usize, value: u8) {
        if let Some(ref mut step) = self.current_step {
            step.add_memory_change(offset, value);
        }
    }

    /// Record storage change
    pub fn record_storage_change(&mut self, key: Uint256, old_value: Uint256, new_value: Uint256) {
        if let Some(ref mut step) = self.current_step {
            step.add_storage_change(key, old_value, new_value);
        }
    }

    /// Add metadata to current step
    pub fn add_metadata(&mut self, key: String, value: String) {
        if let Some(ref mut step) = self.current_step {
            step.add_metadata(key, value);
        }
    }

    /// Finalize the trace
    pub fn finalize(mut self, success: bool, total_gas_consumed: u64) -> ExecutionTrace {
        let execution_time = self.start_time.elapsed();
        self.trace.finalize(success, total_gas_consumed, execution_time.as_micros() as u64);
        self.trace
    }

    /// Get current trace (for inspection)
    pub fn get_trace(&self) -> &ExecutionTrace {
        &self.trace
    }
}

impl Default for ExecutionTracer {
    fn default() -> Self {
        ExecutionTracer::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opcodes::Opcode;

    #[test]
    fn test_execution_step_creation() {
        let step = ExecutionStep::new(
            0,
            Opcode::Add,
            vec![Uint256::from_u32(2), Uint256::from_u32(3)],
            vec![Uint256::from_u32(5)],
            3,
            97,
            0,
        );
        
        assert_eq!(step.pc, 0);
        assert_eq!(step.opcode, Opcode::Add);
        assert_eq!(step.stack_difference(), -1);
        assert!(!step.modified_storage());
        assert!(!step.modified_memory());
    }

    #[test]
    fn test_execution_trace() {
        let mut trace = ExecutionTrace::new();
        
        let step1 = ExecutionStep::new(0, Opcode::Push1, vec![], vec![Uint256::from_u32(1)], 3, 97, 0);
        let step2 = ExecutionStep::new(1, Opcode::Add, vec![Uint256::from_u32(1)], vec![Uint256::from_u32(2)], 3, 94, 0);
        
        trace.add_step(step1);
        trace.add_step(step2);
        trace.finalize(true, 6, 1000);
        
        assert_eq!(trace.steps.len(), 2);
        assert_eq!(trace.opcode_count, 2);
        assert_eq!(trace.opcode_frequency.len(), 2);
        assert!(trace.success);
    }

    #[test]
    fn test_execution_tracer() {
        let mut tracer = ExecutionTracer::new();
        
        tracer.start_step(0, Opcode::Push1, vec![], 100, 0);
        tracer.end_step(vec![Uint256::from_u32(1)], 3, 97);
        
        let trace = tracer.finalize(true, 3);
        assert_eq!(trace.steps.len(), 1);
        assert_eq!(trace.steps[0].opcode, Opcode::Push1);
    }
}
