use rusqlite::{params, Result};
use super::connect::initialize_database;
use super::super::controllers::transaction_controller::Transaction;

pub fn create(amount: f64, description: String, date: String, budget_group_id: i32) -> Result<i32> {
    let conn = initialize_database().unwrap();

    conn.execute(
        "INSERT INTO transactions (amount, description, date, budget_group_id) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![amount, description, date, budget_group_id],
    )?;

    Ok(conn.last_insert_rowid() as i32)
}

pub fn get_all() -> Result<Vec<Transaction>> {
    let conn = initialize_database().unwrap();
    
    let mut stmt = conn.prepare("SELECT * FROM transactions")?;

    let transaction_iter = stmt.query_map([], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            amount: row.get(1)?,
            description: row.get(2)?,
            date: row.get(3)?,
            budget_group_id: row.get(4)?,
        })
    })?;

    let mut transactions = Vec::new();

    for transaction in transaction_iter {
        transactions.push(transaction?);
    }

    Ok(transactions)
}