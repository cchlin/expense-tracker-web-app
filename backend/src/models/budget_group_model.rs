use rusqlite::{params, Connection, Result};
use crate::models::connect;

pub struct BudgetGroup {
    id: i32,
    name: String,
    budget_amount: f64,
    remaining_budget: f64,
}

impl BudgetGroup {
    // create a new budget group
    pub fn create(name: String, budget_amount: f64) -> Result<usize> {
        let conn = connect::initialize_database().unwrap();
        conn.execute(
            "INSERT INTO budget_groups (name, budget_amount, remaining_budget) VALUES(?1, ?2, ?3)",
            params![name, budget_amount, budget_amount],
        )?;

        // Get the id of the last inserted row
        let id = conn.last_insert_rowid();
        Ok(id)
    }
}