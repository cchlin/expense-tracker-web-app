use rusqlite::{params, Connection, Result};
use crate::models::connect;

pub struct Transaction {
    id: i32,
    amount: f64,
    description: String,
    created_at: String,
    updated_at: String,
    budget_group_id: i32,
}

impl Transaction {
    pub fn create(amount: f64, description: String, created_at: String, updated_at: String, budget_group_id: i32) -> Result<usize> {
        let conn = connect::initialize_database().unwrap();

        conn.execute(
            "INSERT INTO transactions (amount, description, created_at, updated_at, budget_group_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![amount, description, created_at, updated_at, budget_group_id],
        )?;

        Ok(conn.last_insert_rowid() as usize)
    }
}