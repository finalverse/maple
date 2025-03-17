// storage/mapledb/src/lib.rs
// Copyright © 2025 Finalverse Inc. <maple@finalverse.com>
// Official Website: https://mapleai.org
// GitHub: https://github.com/finalverse/mapleai.git

use sqlite::{Connection, State};
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use std::sync::Arc;

// Custom error type for better error handling
#[derive(Debug)]
pub enum MapleDBError {
    SQLite(sqlite::Error),
    String(String), // For custom messages if needed
}

impl From<sqlite::Error> for MapleDBError {
    fn from(err: sqlite::Error) -> Self {
        MapleDBError::SQLite(err)
    }
}

impl std::fmt::Display for MapleDBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapleDBError::SQLite(e) => write!(f, "SQLite error: {}", e),
            MapleDBError::String(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for MapleDBError {}

// Custom Result type alias
type Result<T> = std::result::Result<T, MapleDBError>;

// Macro to simplify error mapping
macro_rules! try_sql {
    ($expr:expr) => {
        $expr.map_err(MapleDBError::SQLite)?
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub status: String, // e.g., "active", "completed"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentDef {
    pub id: String,
    pub role: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub role: String, // "admin" or "user"
}

pub struct MapleDB {
    conn: Arc<Mutex<Connection>>, // Thread-safe connection
}

impl MapleDB {
    /// Initialize a new MapleDB instance with SQLite connection
    pub fn new(path: &str) -> Result<Self> {
        let conn = try_sql!(Connection::open(path));
        let conn = Arc::new(Mutex::new(conn));
        {
            // Lock for initialization
            let mut conn = conn.lock().await; // Async lock
            
            // initiate users
            try_sql!(conn.execute(
                "CREATE TABLE IF NOT EXISTS users (
                    id TEXT PRIMARY KEY,
                    role TEXT NOT NULL
                )"
            ));

            // initiate projects
            try_sql!(conn.execute(
                "CREATE TABLE IF NOT EXISTS projects (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    description TEXT,
                    status TEXT NOT NULL
                )"
            ));
            
            // initiate agents
            try_sql!(conn.execute(
                "CREATE TABLE IF NOT EXISTS agents (
                    id TEXT PRIMARY KEY,
                    role TEXT NOT NULL,
                    description TEXT
                )"
            ));
            
            // initiate logs
            try_sql!(conn.execute(
                "CREATE TABLE IF NOT EXISTS logs (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    event TEXT
                )"
            ));
        } // Lock released here
        Ok(MapleDB { conn })
    }

    // user management
    pub fn add_user(&self, id: &str, role: &str) -> Result<()> {
        // Use async block to handle mutex locking
        let conn = self.conn.clone();
        let mut conn = futures::executor::block_on(conn.lock()); // Blocking for simplicity; see notes

        let mut stmt = try_sql!(conn.prepare("INSERT INTO users (id, role) VALUES (?, ?)"));
        try_sql!(stmt.bind((1, id)));
        try_sql!(stmt.bind((2, role)));
        try_sql!(stmt.next());
        Ok(())
    }

    pub fn get_user_role(&self, id: &str) -> Result<Option<String>> {
        // Use async block to handle mutex locking
        let conn = self.conn.clone();
        let mut conn = futures::executor::block_on(conn.lock()); // Blocking for simplicity; see notes

        let mut stmt = try_sql!(conn.prepare("SELECT role FROM users WHERE id = ?"));
        try_sql!(stmt.bind((1, id)));
        match try_sql!(stmt.next()) {
            State::Row => Ok(Some(try_sql!(stmt.read::<String, _>(0)))),
            _ => Ok(None),
        }
    }
    
    // Project Management

    /// Create a new project and return its ID
    pub fn create_project(&self, name: &str, description: &str) -> Result<i64> {
        // Use async block to handle mutex locking
        let conn = self.conn.clone();
        let mut conn = futures::executor::block_on(conn.lock()); // Blocking for simplicity; see notes

        let mut stmt = try_sql!(conn.prepare(
            "INSERT INTO projects (name, description, status) VALUES (?, ?, 'active')"
        ));
        try_sql!(stmt.bind((1, name)));
        try_sql!(stmt.bind((2, description)));
        try_sql!(stmt.next());
        // Use SQLite's last_insert_row_id via a query since direct method isn’t available
        let mut stmt = try_sql!(conn.prepare("SELECT last_insert_rowid()"));
        try_sql!(stmt.next());
        let id = try_sql!(stmt.read::<i64, _>(0));
        Ok(id)
    }

    /// List all projects in the database
    pub fn list_projects(&self) -> Result<Vec<Project>> {
        // Use async block to handle mutex locking
        let conn = self.conn.clone();
        let mut conn = futures::executor::block_on(conn.lock()); // Blocking for simplicity; see notes

        let mut stmt = try_sql!(conn.prepare("SELECT id, name, description, status FROM projects"));
        let mut projects = Vec::new();
        while let State::Row = try_sql!(stmt.next()) {
            projects.push(Project {
                id: try_sql!(stmt.read::<i64, _>(0)),
                name: try_sql!(stmt.read::<String, _>(1)),
                description: try_sql!(stmt.read::<String, _>(2)),
                status: try_sql!(stmt.read::<String, _>(3)),
            });
        }
        Ok(projects)
    }

    /// Switch the status of a project (e.g., "active" to "completed")
    pub fn switch_project_status(&self, id: i64, status: &str) -> Result<()> {
        // Use async block to handle mutex locking
        let conn = self.conn.clone();
        let mut conn = futures::executor::block_on(conn.lock()); // Blocking for simplicity; see notes

        let mut stmt = try_sql!(conn.prepare("UPDATE projects SET status = ? WHERE id = ?"));
        try_sql!(stmt.bind((1, status)));
        try_sql!(stmt.bind((2, id)));
        try_sql!(stmt.next());
        Ok(())
    }

    /// Delete a project by ID
    pub fn delete_project(&self, id: i64) -> Result<()> {
        // Use async block to handle mutex locking
        let conn = self.conn.clone();
        let mut conn = futures::executor::block_on(conn.lock()); // Blocking for simplicity; see notes

        // Prepare a DELETE statement targeting the project with the given ID
        let mut stmt = try_sql!(conn.prepare("DELETE FROM projects WHERE id = ?"));
        // Bind the ID parameter to the statement
        try_sql!(stmt.bind((1, id)));
        // Execute the deletion
        try_sql!(stmt.next());
        // Check if any rows were affected to confirm deletion
        let changes = conn.change_count();
        if changes == 0 {
            return Err(MapleDBError::String(format!("No project found with id {}", id)));
        }
        Ok(())
    }

    // Agent Management

    /// Define or update an agent with a unique ID
    pub fn define_agent(&self, id: &str, role: &str, description: &str) -> Result<()> {
        // Use async block to handle mutex locking
        let conn = self.conn.clone();
        let mut conn = futures::executor::block_on(conn.lock()); // Blocking for simplicity; see notes

        let mut stmt = try_sql!(conn.prepare(
            "INSERT OR REPLACE INTO agents (id, role, description) VALUES (?, ?, ?)"
        ));
        try_sql!(stmt.bind((1, id)));
        try_sql!(stmt.bind((2, role)));
        try_sql!(stmt.bind((3, description)));
        try_sql!(stmt.next());
        Ok(())
    }

    /// List all defined agents
    pub fn list_agents(&self) -> Result<Vec<AgentDef>> {
        // Use async block to handle mutex locking
        let conn = self.conn.clone();
        let mut conn = futures::executor::block_on(conn.lock()); // Blocking for simplicity; see notes

        let mut stmt = try_sql!(conn.prepare("SELECT id, role, description FROM agents"));
        let mut agents = Vec::new();
        while let State::Row = try_sql!(stmt.next()) {
            agents.push(AgentDef {
                id: try_sql!(stmt.read::<String, _>(0)),
                role: try_sql!(stmt.read::<String, _>(1)),
                description: try_sql!(stmt.read::<String, _>(2)),
            });
        }
        Ok(agents)
    }

    /// Delete an agent by ID
    pub fn delete_agent(&self, id: &str) -> Result<()> {
        // Use async block to handle mutex locking
        let conn = self.conn.clone();
        let mut conn = futures::executor::block_on(conn.lock()); // Blocking for simplicity; see notes

        // Prepare a DELETE statement targeting the agent with the given ID
        let mut stmt = try_sql!(conn.prepare("DELETE FROM agents WHERE id = ?"));
        // Bind the ID parameter to the statement
        try_sql!(stmt.bind((1, id)));
        // Execute the deletion
        try_sql!(stmt.next());
        // Check if any rows were affected to confirm deletion
        let changes = conn.change_count();
        if changes == 0 {
            return Err(MapleDBError::String(format!("No agent found with id {}", id)));
        }
        Ok(())
    }

    // Log Management

    /// Log an event to the database
    pub fn log_event(&self, event: &str) -> Result<()> {
        // Use async block to handle mutex locking
        let conn = self.conn.clone();
        let mut conn = futures::executor::block_on(conn.lock()); // Blocking for simplicity; see notes
        let mut stmt = try_sql!(conn.prepare("INSERT INTO logs (event) VALUES (?)"));
        try_sql!(stmt.bind((1, event)));
        try_sql!(stmt.next());
        Ok(())
    }

    /// Retrieve all logged events
    pub fn get_logs(&self) -> Result<Vec<String>> {
        // Use async block to handle mutex locking
        let conn = self.conn.clone();
        let mut conn = futures::executor::block_on(conn.lock()); // Blocking for simplicity; see notes

        let mut stmt = try_sql!(conn.prepare("SELECT event FROM logs"));
        let mut logs = Vec::new();
        while let State::Row = try_sql!(stmt.next()) {
            logs.push(try_sql!(stmt.read::<String, _>(0)));
        }
        Ok(logs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_crud() {
        let db = MapleDB::new(":memory:").unwrap();
        let id = db.create_project("Test Project", "A test project").unwrap();
        assert!(id > 0);

        let projects = db.list_projects().unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].name, "Test Project");

        db.switch_project_status(id, "completed").unwrap();
        let projects = db.list_projects().unwrap();
        assert_eq!(projects[0].status, "completed");

        db.delete_project(id).unwrap();
        let projects = db.list_projects().unwrap();
        assert_eq!(projects.len(), 0);
    }

    #[test]
    fn test_agent_crud() {
        let db = MapleDB::new(":memory:").unwrap();
        db.define_agent("agent1", "tester", "Test agent").unwrap();

        let agents = db.list_agents().unwrap();
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].id, "agent1");

        db.delete_agent("agent1").unwrap();
        let agents = db.list_agents().unwrap();
        assert_eq!(agents.len(), 0);
    }
}