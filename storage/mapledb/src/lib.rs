// MapleDB data layer for agent state and metadata in MAPLE
// © 2025 Finalverse Inc. All rights reserved.

use sled::{Db, Error as SledError};
use std::path::Path;

/// MapleDB instance for storing agent data
pub struct MapleDb {
    db: Db,
}

impl MapleDb {
    /// Opens or creates a new MapleDB instance at the specified path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, SledError> {
        let db = sled::open(path)?;
        Ok(MapleDb { db })
    }

    /// Stores a key-value pair
    pub fn store(&self, key: &str, value: &[u8]) -> Result<(), SledError> {
        self.db.insert(key.as_bytes(), value)?;
        self.db.flush()?;
        Ok(())
    }

    /// Retrieves a value by key
    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, SledError> {
        self.db
            .get(key.as_bytes())
            .map(|opt| opt.map(|v| v.to_vec()))
    }

    /// Deletes a key-value pair
    pub fn delete(&self, key: &str) -> Result<(), SledError> {
        self.db.remove(key.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapledb() {
        let db = MapleDb::new("test_mapledb").unwrap();
        db.store("agent1", b"data").unwrap();
        let value = db.get("agent1").unwrap().unwrap();
        assert_eq!(value, b"data");
        db.delete("agent1").unwrap();
        assert!(db.get("agent1").unwrap().is_none());
    }
}