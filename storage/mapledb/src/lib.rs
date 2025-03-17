// storage/mapledb/src/lib.rs
// Copyright © 2025 Finalverse Inc. <maple@finalverse.com>
// Official Website: https://mapleai.org
// GitHub: https://github.com/finalverse/mapleai.git

use sqlite::{Connection, State}; // Import necessary SQLite types

pub struct MapleDB {
    conn: Connection, // SQLite connection handle
}

impl MapleDB {
    /// Initialize a new MapleDB with an SQLite connection
    /// - `path`: File path to the SQLite database (e.g., "maple.db")
    /// - Returns `Result<Self, String>` with a custom error message on failure
    pub fn new(path: &str) -> Result<Self, String> {
        // Open SQLite connection; map any error to a string for simplicity
        let conn = Connection::open(path).map_err(|e| format!("Failed to open database: {}", e))?;

        // Create logs table if it doesn’t exist; id is auto-incrementing, event stores text
        conn.execute("CREATE TABLE IF NOT EXISTS logs (id INTEGER PRIMARY KEY, event TEXT)")
            .map_err(|e| format!("Failed to create table: {}", e))?;

        // Return the initialized MapleDB instance
        Ok(MapleDB { conn })
    }

    /// Log an event (e.g., UAL execution) into the database
    /// - `event`: The event string to log
    /// - Returns `Result<(), String>` with an error message on failure
    pub fn log_event(&self, event: &str) -> Result<(), String> {
        // Prepare an INSERT statement with a placeholder for the event
        let mut stmt = self
            .conn
            .prepare("INSERT INTO logs (event) VALUES (?)")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        // Bind the event string to the first placeholder (index starts at 1 in SQLite)
        stmt.bind((1, event))
            .map_err(|e| format!("Failed to bind event: {}", e))?;

        // Execute the statement and move to the next state (should be Done for INSERT)
        stmt.next()
            .map_err(|e| format!("Failed to execute insert: {}", e))?;

        Ok(())
    }

    /// Retrieve all logged events from the database
    /// - Returns `Result<Vec<String>, String>` with events or an error message
    pub fn get_logs(&self) -> Result<Vec<String>, String> {
        // Prepare a SELECT statement to fetch all events
        let mut stmt = self
            .conn
            .prepare("SELECT event FROM logs")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let mut logs = Vec::new();
        // Iterate over rows until no more are available
        while let State::Row = stmt
            .next()
            .map_err(|e| format!("Failed to fetch row: {}", e))?
        {
            // Read the event column (index 0) as a String
            let event = stmt
                .read::<String, _>(0)
                .map_err(|e| format!("Failed to read event: {}", e))?;
            logs.push(event);
        }

        Ok(logs)
    }
}