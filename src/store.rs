use crate::config::Config;
use crate::lsmt::{LSMTError, LSMT};
use std::collections::HashMap;
use std::hash::Hash;

/// Depicts whether an operation was successfully executed or not.
#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum ExecResult {
    Success,
    Failed,
}

/// The Storage Engine
pub struct Store<A, B> {
    /// A KV store in the form of in-memory HashMap.
    /// Types A and B can be defined by the use case.
    storage: HashMap<A, B>,
}

/// As is clear from the implementation, types A and B must implement Display
/// to be 'printable'. While A must also implement Hash and Eq traits
impl<A: Hash + Eq, B: Clone> Store<A, B> {
    /// Creates a new Storage Engine.
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    /// Operates HashMap::insert()
    pub fn set(&mut self, key: A, value: B) -> ExecResult {
        // Fails if key already points to another value, else stores key-value pair and returns success.
        if self.storage.contains_key(&key) {
            eprintln!("Error: Key already associated with another value.");
            ExecResult::Failed
        } else {
            self.storage.insert(key, value);
            ExecResult::Success
        }
    }

    /// Operates HashMap::get() and fails if key-value pair doesn't
    /// exist, else returns value on success.
    pub fn get(&self, key: A) -> Result<B, ExecResult> {
        match self.storage.get(&key) {
            None => Err(ExecResult::Failed),
            Some(s) => Ok(B::from(s.clone())),
        }
    }

    /// Operates HashMap::remove() and fails if the key-value pair
    /// doesn't exist, else deletes it and returns success.
    pub fn del(&mut self, key: A) -> ExecResult {
        match self.storage.remove(&key) {
            Some(val) => {
                println!("Deleted: Key -> Value mapping.");
                ExecResult::Success
            }
            None => {
                eprintln!("Error: Can't remove, as no value associated with key.");
                ExecResult::Failed
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_key_not_found() {
        let store: Store<&str, &str> = Store::new();

        let result = store.get("key1");
        assert_eq!(result, Err(ExecResult::Failed));
    }

    #[test]
    fn test_set_key_in_use() {
        let mut store = Store::new();

        let result = store.set("key1", "value1");
        assert_eq!(result, ExecResult::Success);

        let result = store.set("key1", "value1");
        assert_eq!(result, ExecResult::Failed);
    }

    #[test]
    fn test_del_key_not_found() {
        let mut store: Store<&str, &str> = Store::new();

        let result = store.del("key1");
        assert_eq!(result, ExecResult::Failed);
    }

    #[test]
    fn test_flow_ok() {
        let mut store = Store::new();

        let result = store.set("key1", "value1");
        assert_eq!(result, ExecResult::Success);

        let result = store.get("key1");
        assert_eq!(result, Ok("value1"));

        let result = store.del("key1");
        assert_eq!(result, ExecResult::Success);

        let result = store.get("key1");
        assert_eq!(result, Err(ExecResult::Failed));
    }
}
