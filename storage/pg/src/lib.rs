// PostgreSQL backend for structured data storage in MAPLE
// © 2025 Finalverse Inc. All rights reserved.

use sqlx::{Pool, Postgres, Error as SqlxError};
use std::env;

/// PostgreSQL storage instance
pub struct PgStorage {
    pool: Pool<Postgres>,
}

impl PgStorage {
    /// Connects to a PostgreSQL database
    pub async fn new() -> Result<Self, SqlxError> {
        let url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/maple".to_string());
        let pool = Pool::connect(&url).await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS agents (
                id TEXT PRIMARY KEY,
                data JSONB NOT NULL
            )",
        )
            .execute(&pool)
            .await?;
        Ok(PgStorage { pool })
    }

    /// Stores agent data
    pub async fn store(&self, id: &str, data: &serde_json::Value) -> Result<(), SqlxError> {
        sqlx::query("INSERT INTO agents (id, data) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET data = $2")
            .bind(id)
            .bind(data)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Retrieves agent data
    pub async fn get(&self, id: &str) -> Result<Option<serde_json::Value>, SqlxError> {
        let row = sqlx::query_as::<_, (serde_json::Value,)>("SELECT data FROM agents WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| r.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires PostgreSQL setup
    async fn test_pg_storage() {
        let storage = PgStorage::new().await.unwrap();
        let data = serde_json::json!({"name": "agent1"});
        storage.store("agent1", &data).await.unwrap();
        let retrieved = storage.get("agent1").await.unwrap().unwrap();
        assert_eq!(retrieved["name"], "agent1");
    }
}