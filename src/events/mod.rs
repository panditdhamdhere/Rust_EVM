use crate::types::{Address, Hash, Bytes};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Event log entry
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventLog {
    /// Address that emitted the log
    pub address: Address,
    /// Topics (indexed parameters)
    pub topics: Vec<Hash>,
    /// Data (non-indexed parameters)
    pub data: Bytes,
}

impl EventLog {
    /// Create a new event log
    pub fn new(address: Address, topics: Vec<Hash>, data: Bytes) -> Self {
        EventLog {
            address,
            topics,
            data,
        }
    }
}

impl fmt::Display for EventLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Log from {} with {} topics, {} bytes data", 
            self.address, self.topics.len(), self.data.len())
    }
}

/// Event logger for collecting logs during execution
pub struct EventLogger {
    /// List of event logs
    logs: Vec<EventLog>,
}

impl EventLogger {
    /// Create a new event logger
    pub fn new() -> Self {
        EventLogger {
            logs: Vec::new(),
        }
    }

    /// Log an event
    pub fn log(&mut self, address: Address, topics: Vec<Hash>, data: Bytes) {
        self.logs.push(EventLog::new(address, topics, data));
    }

    /// Get all logs
    pub fn logs(&self) -> &[EventLog] {
        &self.logs
    }

    /// Clear all logs
    pub fn clear(&mut self) {
        self.logs.clear();
    }

    /// Get the number of logs
    pub fn count(&self) -> usize {
        self.logs.len()
    }

    /// Get logs for a specific address
    pub fn logs_for_address(&self, address: &Address) -> Vec<&EventLog> {
        self.logs.iter()
            .filter(|log| log.address == *address)
            .collect()
    }

    /// Get logs with a specific topic
    pub fn logs_with_topic(&self, topic: &Hash) -> Vec<&EventLog> {
        self.logs.iter()
            .filter(|log| log.topics.contains(topic))
            .collect()
    }
}

impl Default for EventLogger {
    fn default() -> Self {
        EventLogger::new()
    }
}

/// Log receipt containing execution results and logs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogReceipt {
    /// Whether execution was successful
    pub success: bool,
    /// Gas used
    pub gas_used: u64,
    /// Gas remaining
    pub gas_remaining: u64,
    /// Return data
    pub return_data: Bytes,
    /// Event logs
    pub logs: Vec<EventLog>,
}

impl LogReceipt {
    /// Create a new log receipt
    pub fn new(success: bool, gas_used: u64, gas_remaining: u64, return_data: Bytes, logs: Vec<EventLog>) -> Self {
        LogReceipt {
            success,
            gas_used,
            gas_remaining,
            return_data,
            logs,
        }
    }
}

impl fmt::Display for LogReceipt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Execution {}: {} gas used, {} gas remaining, {} logs", 
            if self.success { "successful" } else { "failed" },
            self.gas_used, self.gas_remaining, self.logs.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_log_creation() {
        let address = Address::zero();
        let topics = vec![Hash::zero()];
        let data = Bytes::from(b"test data".to_vec());
        
        let log = EventLog::new(address, topics.clone(), data.clone());
        
        assert_eq!(log.address, address);
        assert_eq!(log.topics, topics);
        assert_eq!(log.data, data);
    }

    #[test]
    fn test_event_logger() {
        let mut logger = EventLogger::new();
        
        let address = Address::zero();
        let topics = vec![Hash::zero()];
        let data = Bytes::from(b"test".to_vec());
        
        logger.log(address, topics, data);
        
        assert_eq!(logger.count(), 1);
        assert_eq!(logger.logs().len(), 1);
    }

    #[test]
    fn test_log_receipt() {
        let logs = vec![];
        let receipt = LogReceipt::new(
            true,
            1000,
            500,
            Bytes::empty(),
            logs
        );
        
        assert!(receipt.success);
        assert_eq!(receipt.gas_used, 1000);
        assert_eq!(receipt.gas_remaining, 500);
    }
}
