use crate::types::{Address, Uint256, Hash};
use std::collections::HashMap;
use thiserror::Error;
use sha3::{Digest, Keccak256};

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Account not found: {address}")]
    AccountNotFound { address: Address },
    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: Uint256, available: Uint256 },
    #[error("Invalid nonce: expected {expected}, got {got}")]
    InvalidNonce { expected: Uint256, got: Uint256 },
}

/// Account state in the EVM
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    /// Account balance in wei
    pub balance: Uint256,
    /// Account nonce (number of transactions sent)
    pub nonce: Uint256,
    /// Account code (for contracts)
    pub code: Vec<u8>,
    /// Account storage (key-value pairs)
    pub storage: HashMap<Uint256, Uint256>,
    /// Whether this account has been deleted
    pub deleted: bool,
}

impl Account {
    /// Create a new account with zero balance and nonce
    pub fn new() -> Self {
        Account {
            balance: Uint256::zero(),
            nonce: Uint256::zero(),
            code: Vec::new(),
            storage: HashMap::new(),
            deleted: false,
        }
    }

    /// Create a new contract account with code
    pub fn new_contract(code: Vec<u8>) -> Self {
        Account {
            balance: Uint256::zero(),
            nonce: Uint256::zero(),
            code,
            storage: HashMap::new(),
            deleted: false,
        }
    }

    /// Check if this is a contract account (has code)
    pub fn is_contract(&self) -> bool {
        !self.code.is_empty()
    }

    /// Get the code hash of this account
    pub fn code_hash(&self) -> Hash {
        if self.code.is_empty() {
            Hash::zero()
        } else {
            let hash = Keccak256::digest(&self.code);
            Hash::new(*hash.as_ref())
        }
    }

    /// Get a storage value
    pub fn get_storage(&self, key: &Uint256) -> Uint256 {
        self.storage.get(key).cloned().unwrap_or(Uint256::zero())
    }

    /// Set a storage value
    pub fn set_storage(&mut self, key: Uint256, value: Uint256) {
        if value.is_zero() {
            self.storage.remove(&key);
        } else {
            self.storage.insert(key, value);
        }
    }

    /// Add to the account balance
    pub fn add_balance(&mut self, amount: Uint256) {
        self.balance = self.balance.clone() + amount;
    }

    /// Subtract from the account balance
    pub fn sub_balance(&mut self, amount: Uint256) -> Result<(), StorageError> {
        if self.balance < amount {
            return Err(StorageError::InsufficientBalance {
                required: amount,
                available: self.balance.clone(),
            });
        }
        self.balance = self.balance.clone() - amount;
        Ok(())
    }

    /// Increment the account nonce
    pub fn increment_nonce(&mut self) {
        self.nonce = self.nonce.clone() + Uint256::one();
    }

    /// Mark the account as deleted
    pub fn delete(&mut self) {
        self.deleted = true;
        self.balance = Uint256::zero();
        self.nonce = Uint256::zero();
        self.code.clear();
        self.storage.clear();
    }
}

impl Default for Account {
    fn default() -> Self {
        Account::new()
    }
}

/// EVM Storage - manages account states and storage
pub struct Storage {
    /// Map of address to account state
    accounts: HashMap<Address, Account>,
}

impl Storage {
    /// Create a new storage instance
    pub fn new() -> Self {
        Storage {
            accounts: HashMap::new(),
        }
    }

    /// Get an account, creating it if it doesn't exist
    pub fn get_or_create_account(&mut self, address: Address) -> &mut Account {
        self.accounts.entry(address).or_insert_with(Account::new)
    }

    /// Get an account reference
    pub fn get_account(&self, address: &Address) -> Option<&Account> {
        self.accounts.get(address)
    }

    /// Get an account mutable reference
    pub fn get_account_mut(&mut self, address: &Address) -> Option<&mut Account> {
        self.accounts.get_mut(address)
    }

    /// Check if an account exists
    pub fn account_exists(&self, address: &Address) -> bool {
        self.accounts.contains_key(address)
    }

    /// Delete an account
    pub fn delete_account(&mut self, address: &Address) {
        if let Some(account) = self.accounts.get_mut(address) {
            account.delete();
        }
    }

    /// Get account balance
    pub fn get_balance(&self, address: &Address) -> Uint256 {
        self.accounts
            .get(address)
            .map(|account| account.balance.clone())
            .unwrap_or(Uint256::zero())
    }

    /// Set account balance
    pub fn set_balance(&mut self, address: Address, balance: Uint256) {
        self.get_or_create_account(address).balance = balance;
    }

    /// Add to account balance
    pub fn add_balance(&mut self, address: Address, amount: Uint256) {
        self.get_or_create_account(address).add_balance(amount);
    }

    /// Subtract from account balance
    pub fn sub_balance(&mut self, address: &Address, amount: Uint256) -> Result<(), StorageError> {
        if let Some(account) = self.accounts.get_mut(address) {
            account.sub_balance(amount)
        } else {
            Err(StorageError::InsufficientBalance {
                required: amount,
                available: Uint256::zero(),
            })
        }
    }

    /// Get account nonce
    pub fn get_nonce(&self, address: &Address) -> Uint256 {
        self.accounts
            .get(address)
            .map(|account| account.nonce.clone())
            .unwrap_or(Uint256::zero())
    }

    /// Set account nonce
    pub fn set_nonce(&mut self, address: Address, nonce: Uint256) {
        self.get_or_create_account(address).nonce = nonce;
    }

    /// Increment account nonce
    pub fn increment_nonce(&mut self, address: Address) {
        self.get_or_create_account(address).increment_nonce();
    }

    /// Get account code
    pub fn get_code(&self, address: &Address) -> Vec<u8> {
        self.accounts
            .get(address)
            .map(|account| account.code.clone())
            .unwrap_or_default()
    }

    /// Set account code
    pub fn set_code(&mut self, address: Address, code: Vec<u8>) {
        self.get_or_create_account(address).code = code;
    }

    /// Get storage value
    pub fn get_storage(&self, address: &Address, key: &Uint256) -> Uint256 {
        self.accounts
            .get(address)
            .map(|account| account.get_storage(key))
            .unwrap_or(Uint256::zero())
    }

    /// Set storage value
    pub fn set_storage(&mut self, address: Address, key: Uint256, value: Uint256) {
        self.get_or_create_account(address).set_storage(key, value);
    }

    /// Get all accounts (for debugging)
    pub fn accounts(&self) -> &HashMap<Address, Account> {
        &self.accounts
    }

    /// Clear all storage
    pub fn clear(&mut self) {
        self.accounts.clear();
    }

    /// Get the number of accounts
    pub fn account_count(&self) -> usize {
        self.accounts.len()
    }
}

impl Default for Storage {
    fn default() -> Self {
        Storage::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_creation() {
        let mut storage = Storage::new();
        let address = Address::zero();
        
        // Account should be created when accessed
        let balance = storage.get_balance(&address);
        assert_eq!(balance, Uint256::zero());
        
        // Set balance
        storage.set_balance(address, Uint256::from_u32(1000));
        assert_eq!(storage.get_balance(&address), Uint256::from_u32(1000));
    }

    #[test]
    fn test_account_balance_operations() {
        let mut storage = Storage::new();
        let address = Address::zero();
        
        storage.add_balance(address, Uint256::from_u32(500));
        assert_eq!(storage.get_balance(&address), Uint256::from_u32(500));
        
        storage.sub_balance(&address, Uint256::from_u32(200)).unwrap();
        assert_eq!(storage.get_balance(&address), Uint256::from_u32(300));
        
        // Test insufficient balance
        assert!(storage.sub_balance(&address, Uint256::from_u32(400)).is_err());
    }

    #[test]
    fn test_account_nonce() {
        let mut storage = Storage::new();
        let address = Address::zero();
        
        assert_eq!(storage.get_nonce(&address), Uint256::zero());
        
        storage.increment_nonce(address);
        assert_eq!(storage.get_nonce(&address), Uint256::one());
    }

    #[test]
    fn test_account_storage() {
        let mut storage = Storage::new();
        let address = Address::zero();
        let key = Uint256::from_u32(42);
        let value = Uint256::from_u32(123);
        
        // Initially zero
        assert_eq!(storage.get_storage(&address, &key), Uint256::zero());
        
        // Set storage
        storage.set_storage(address, key.clone(), value.clone());
        assert_eq!(storage.get_storage(&address, &key), value);
        
        // Set to zero (should remove from storage)
        storage.set_storage(address, key.clone(), Uint256::zero());
        assert_eq!(storage.get_storage(&address, &key), Uint256::zero());
    }

    #[test]
    fn test_contract_account() {
        let mut storage = Storage::new();
        let address = Address::zero();
        let code = b"contract code".to_vec();
        
        storage.set_code(address, code.clone());
        
        let account = storage.get_account(&address).unwrap();
        assert!(account.is_contract());
        assert_eq!(account.code, code);
        assert_ne!(account.code_hash(), Hash::zero());
    }
}
