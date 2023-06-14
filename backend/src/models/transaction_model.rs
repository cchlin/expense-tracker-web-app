use rusqlite::{params, Result};
use super::connect::initialize_database;
use super::super::controllers::transaction_controller::Transaction;

pub fn create(amount: f64, description: String, date: String, budget_group_id: i32) -> Result<i32> {
    let conn = initialize_database().unwrap();

    conn.execute(
        "INSERT INTO transactions (amount, description, date, budget_group_id) VALUES (?1, ?2, ?3, ?4)",
        params![amount, description, date, budget_group_id],
    )?;

    Ok(conn.last_insert_rowid() as i32)
}

pub fn get_all(group_id: i32) -> Result<Vec<Transaction>> {
    let conn = initialize_database().unwrap();
    
    let query = format!("SELECT * FROM transactions WHERE budget_group_id = {}", group_id);
    
    let mut stmt = conn.prepare(&query)?;

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

pub fn delete(id: i32) -> Result<()> {
    let conn = initialize_database().unwrap();

    conn.execute("DELETE FROM transactions WHERE id = ?1", params![id])?;

    Ok(())
}