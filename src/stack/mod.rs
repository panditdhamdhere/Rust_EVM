use crate::types::Uint256;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StackError {
    #[error("Stack overflow: maximum size exceeded")]
    Overflow,
    #[error("Stack underflow: not enough items on stack")]
    Underflow,
}

/// EVM Stack - LIFO data structure with maximum 1024 items
pub struct Stack {
    items: Vec<Uint256>,
    max_size: usize,
}

impl Stack {
    /// Create a new stack with default max size of 1024
    pub fn new() -> Self {
        Stack {
            items: Vec::new(),
            max_size: 1024,
        }
    }

    /// Create a new stack with custom max size
    pub fn with_max_size(max_size: usize) -> Self {
        Stack {
            items: Vec::new(),
            max_size,
        }
    }

    /// Push a value onto the stack
    pub fn push(&mut self, value: Uint256) -> Result<(), StackError> {
        if self.items.len() >= self.max_size {
            return Err(StackError::Overflow);
        }
        self.items.push(value);
        Ok(())
    }

    /// Pop a value from the stack
    pub fn pop(&mut self) -> Result<Uint256, StackError> {
        self.items.pop().ok_or(StackError::Underflow)
    }

    /// Peek at the top value without removing it
    pub fn peek(&self) -> Result<&Uint256, StackError> {
        self.items.last().ok_or(StackError::Underflow)
    }

    /// Peek at a value at a specific depth (0 = top)
    pub fn peek_at(&self, depth: usize) -> Result<&Uint256, StackError> {
        if depth >= self.items.len() {
            return Err(StackError::Underflow);
        }
        Ok(&self.items[self.items.len() - 1 - depth])
    }

    /// Get the current stack size
    pub fn size(&self) -> usize {
        self.items.len()
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get the maximum stack size
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    /// Duplicate the top item
    pub fn dup(&mut self, depth: usize) -> Result<(), StackError> {
        if depth >= self.items.len() {
            return Err(StackError::Underflow);
        }
        let value = self.items[self.items.len() - 1 - depth].clone();
        self.push(value)?;
        Ok(())
    }

    /// Swap the top item with the item at depth
    pub fn swap(&mut self, depth: usize) -> Result<(), StackError> {
        if depth >= self.items.len() {
            return Err(StackError::Underflow);
        }
        let len = self.items.len();
        self.items.swap(len - 1, len - 1 - depth);
        Ok(())
    }

    /// Clear the stack
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Get all items as a slice (for debugging)
    pub fn items(&self) -> &[Uint256] {
        &self.items
    }
}

impl Default for Stack {
    fn default() -> Self {
        Stack::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::new();
        let value = Uint256::from_u32(42);
        
        assert!(stack.push(value.clone()).is_ok());
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.pop().unwrap(), value);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_stack_overflow() {
        let mut stack = Stack::with_max_size(2);
        
        assert!(stack.push(Uint256::from_u32(1)).is_ok());
        assert!(stack.push(Uint256::from_u32(2)).is_ok());
        assert!(stack.push(Uint256::from_u32(3)).is_err());
    }

    #[test]
    fn test_stack_underflow() {
        let mut stack = Stack::new();
        assert!(stack.pop().is_err());
        assert!(stack.peek().is_err());
    }

    #[test]
    fn test_stack_dup() {
        let mut stack = Stack::new();
        let value1 = Uint256::from_u32(1);
        let value2 = Uint256::from_u32(2);
        
        stack.push(value1.clone()).unwrap();
        stack.push(value2.clone()).unwrap();
        
        // DUP1 should duplicate the top item
        stack.dup(0).unwrap();
        assert_eq!(stack.size(), 3);
        assert_eq!(stack.pop().unwrap(), value2);
        assert_eq!(stack.pop().unwrap(), value2);
        assert_eq!(stack.pop().unwrap(), value1);
    }

    #[test]
    fn test_stack_swap() {
        let mut stack = Stack::new();
        let value1 = Uint256::from_u32(1);
        let value2 = Uint256::from_u32(2);
        let value3 = Uint256::from_u32(3);
        
        stack.push(value1.clone()).unwrap();
        stack.push(value2.clone()).unwrap();
        stack.push(value3.clone()).unwrap();
        
        // SWAP1 should swap top with second item
        stack.swap(1).unwrap();
        assert_eq!(stack.pop().unwrap(), value2);
        assert_eq!(stack.pop().unwrap(), value3);
        assert_eq!(stack.pop().unwrap(), value1);
    }
}
