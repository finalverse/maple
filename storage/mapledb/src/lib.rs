// storage/mapledb/src/lib.rs
// Copyright © 2025 Finalverse Inc. <maple@finalverse.com>
// Official Website: https://mapleai.org
// GitHub: https://github.com/finalverse/mapleai.git

use sqlite::{Connection, State};
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;
use std::sync::Arc;

// Custom error type for database operations
#[derive(Debug)]
pub enum MapleDBError {
    SQLite(sqlite::Error),      // Wraps SQLite-specific errors
    String(String),             // Custom error messages
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

// Result type alias for convenience
type Result<T> = std::result::Result<T, MapleDBError>;

// Macro to simplify SQLite error handling
macro_rules! try_sql {
    ($expr:expr) => {
        $expr.map_err(MapleDBError::SQLite)?
    };
}

// Project structure for project management
#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub status: String, // e.g., "active", "completed"
}

// Agent definition structure for agents management
#[derive(Serialize, Deserialize, Debug)]
pub struct AgentDef {
    pub id: String,
    pub role: String,
    pub description: String,
}

// User structure for permission management
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub role: String, // "admin" or "user"
}

#[derive(Clone)]
pub struct MapleDB {
    conn: Arc<Mutex<Connection>>, // Thread-safe SQLite connection
}

impl MapleDB {
    /// Initialize a new MapleDB instance synchronously for compatibility
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?; // Synchronous open
        let conn = Arc::new(Mutex::new(conn));
        let db = MapleDB { conn };

        // Run initialization in a blocking Tokio runtime
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(db.init_tables())?;

        Ok(db)
    }

    /// Initialize database tables asynchronously
    async fn init_tables(&self) -> Result<()> {
        let mut conn = self.conn.lock().await; // Async lock

        // Create users table
        try_sql!(conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                role TEXT NOT NULL
            )"
        ));

        // Create projects table
        try_sql!(conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL
            )"
        ));

        // Create agents table
        try_sql!(conn.execute(
            "CREATE TABLE IF NOT EXISTS agents (
                id TEXT PRIMARY KEY,
                role TEXT NOT NULL,
                description TEXT
            )"
        ));

        // Create logs table
        try_sql!(conn.execute(
            "CREATE TABLE IF NOT EXISTS logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                event TEXT
            )"
        ));

        Ok(())
    }

    // User Management

    /// Add a new user with a role asynchronously
    pub async fn add_user(&self, id: &str, role: &str) -> Result<()> {
        let mut conn = self.conn.lock().await;
        let mut stmt = try_sql!(conn.prepare("INSERT INTO users (id, role) VALUES (?, ?)"));
        try_sql!(stmt.bind((1, id)));
        try_sql!(stmt.bind((2, role)));
        try_sql!(stmt.next());
        Ok(())
    }

    /// Get a user's role by ID asynchronously
    pub async fn get_user_role(&self, id: &str) -> Result<Option<String>> {
        let mut conn = self.conn.lock().await;
        let mut stmt = try_sql!(conn.prepare("SELECT role FROM users WHERE id = ?"));
        try_sql!(stmt.bind((1, id)));
        match try_sql!(stmt.next()) {
            State::Row => Ok(Some(try_sql!(stmt.read::<String, _>(0)))),
            _ => Ok(None),
        }
    }

    // Project Management

    /// Create a new project and return its ID asynchronously
    pub async fn create_project(&self, name: &str, description: &str) -> Result<i64> {
        let mut conn = self.conn.lock().await;
        let mut stmt = try_sql!(conn.prepare(
            "INSERT INTO projects (name, description, status) VALUES (?, ?, 'active')"
        ));
        try_sql!(stmt.bind((1, name)));
        try_sql!(stmt.bind((2, description)));
        try_sql!(stmt.next());
        let mut stmt = try_sql!(conn.prepare("SELECT last_insert_rowid()"));
        try_sql!(stmt.next());
        let id = try_sql!(stmt.read::<i64, _>(0));
        Ok(id)
    }

    /// List all projects asynchronously
    pub async fn list_projects(&self) -> Result<Vec<Project>> {
        let mut conn = self.conn.lock().await;
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

    /// Switch a project's status asynchronously
    pub async fn switch_project_status(&self, id: i64, status: &str) -> Result<()> {
        let mut conn = self.conn.lock().await;
        let mut stmt = try_sql!(conn.prepare("UPDATE projects SET status = ? WHERE id = ?"));
        try_sql!(stmt.bind((1, status)));
        try_sql!(stmt.bind((2, id)));
        try_sql!(stmt.next());
        Ok(())
    }

    /// Delete a project by ID asynchronously
    pub async fn delete_project(&self, id: i64) -> Result<()> {
        let mut conn = self.conn.lock().await;
        let mut stmt = try_sql!(conn.prepare("DELETE FROM projects WHERE id = ?"));
        try_sql!(stmt.bind((1, id)));
        try_sql!(stmt.next());
        let changes = conn.change_count();
        if changes == 0 {
            return Err(MapleDBError::String(format!("No project found with id {}", id)));
        }
        Ok(())
    }

    // Agent Management

    /// Define or update an agents asynchronously
    pub async fn define_agent(&self, id: &str, role: &str, description: &str) -> Result<()> {
        let mut conn = self.conn.lock().await;
        let mut stmt = try_sql!(conn.prepare(
            "INSERT OR REPLACE INTO agents (id, role, description) VALUES (?, ?, ?)"
        ));
        try_sql!(stmt.bind((1, id)));
        try_sql!(stmt.bind((2, role)));
        try_sql!(stmt.bind((3, description)));
        try_sql!(stmt.next());
        Ok(())
    }

    /// List all agents asynchronously
    pub async fn list_agents(&self) -> Result<Vec<AgentDef>> {
        let mut conn = self.conn.lock().await;
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

    /// Delete an agents by ID asynchronously
    pub async fn delete_agent(&self, id: &str) -> Result<()> {
        let mut conn = self.conn.lock().await;
        let mut stmt = try_sql!(conn.prepare("DELETE FROM agents WHERE id = ?"));
        try_sql!(stmt.bind((1, id)));
        try_sql!(stmt.next());
        let changes = conn.change_count();
        if changes == 0 {
            return Err(MapleDBError::String(format!("No agents found with id {}", id)));
        }
        Ok(())
    }

    // Log Management

    /// Log an event asynchronously
    pub async fn log_event(&self, event: &str) -> Result<()> {
        let mut conn = self.conn.lock().await;
        let mut stmt = try_sql!(conn.prepare("INSERT INTO logs (event) VALUES (?)"));
        try_sql!(stmt.bind((1, event)));
        try_sql!(stmt.next());
        Ok(())
    }

    /// Retrieve all logs asynchronously
    pub async fn get_logs(&self) -> Result<Vec<String>> {
        let mut conn = self.conn.lock().await;
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

    #[tokio::test]
    async fn test_project_crud() {
        let db = MapleDB::new(":memory:").unwrap();
        let id = db.create_project("Test Project", "A test project").await.unwrap();
        assert!(id > 0);

        let projects = db.list_projects().await.unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].name, "Test Project");

        db.switch_project_status(id, "completed").await.unwrap();
        let projects = db.list_projects().await.unwrap();
        assert_eq!(projects[0].status, "completed");

        db.delete_project(id).await.unwrap();
        let projects = db.list_projects().await.unwrap();
        assert_eq!(projects.len(), 0);
    }

    #[tokio::test]
    async fn test_agent_crud() {
        let db = MapleDB::new(":memory:").unwrap();
        db.define_agent("agent1", "tester", "Test agents").await.unwrap();

        let agents = db.list_agents().await.unwrap();
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].id, "agent1");

        db.delete_agent("agent1").await.unwrap();
        let agents = db.list_agents().await.unwrap();
        assert_eq!(agents.len(), 0);
    }
}