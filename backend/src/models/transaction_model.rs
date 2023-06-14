use super::super::controllers::transaction_controller::Transaction;
use super::connect::initialize_database;
use rusqlite::{params, Result};

/// Inserts a new transaction into the database.
///
/// # Arguments
///
/// * `amount` - A floating point that holds the amount of the transaction.
/// * `description` - A string that describes the transaction.
/// * `date` - A string that holds the date of the transaction.
/// * `budget_group_id` - An i32 that holds the budget group ID of the transaction.
///
/// # Returns
///
/// Returns an i32 as the ID of the new transaction.
pub fn create(amount: f64, description: String, date: String, budget_group_id: i32) -> Result<i32> {
    // Establishing a connection with the database
    let conn = initialize_database().unwrap();

    // Executing the SQL query to insert the new transaction
    conn.execute(
        "INSERT INTO transactions (amount, description, date, budget_group_id) VALUES (?1, ?2, ?3, ?4)",
        params![amount, description, date, budget_group_id],
    )?;

    // Returning the ID of the last inserted transaction
    Ok(conn.last_insert_rowid() as i32)
}

/// Retrieves all transactions for a specific budget group from the database.
///
/// # Arguments
///
/// * `group_id` - An i32 that holds the ID of the budget group.
///
/// # Returns
///
/// Returns a Vector of `Transaction` objects.
pub fn get_all(group_id: i32) -> Result<Vec<Transaction>> {
    // Establishing a connection with the database
    let conn = initialize_database().unwrap();

    // Constructing the SQL query
    let query = format!(
        "SELECT * FROM transactions WHERE budget_group_id = {}",
        group_id
    );

    // Preparing the SQL statement
    let mut stmt = conn.prepare(&query)?;

    // Executing the query and mapping the results to a list of transactions
    let transaction_iter = stmt.query_map([], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            amount: row.get(1)?,
            description: row.get(2)?,
            date: row.get(3)?,
            budget_group_id: row.get(4)?,
        })
    })?;

    // Collecting the transactions into a vector
    let mut transactions = Vec::new();
    for transaction in transaction_iter {
        transactions.push(transaction?);
    }

    // Returning the vector of transactions
    Ok(transactions)
}

/// Deletes a specific transaction from the database.
///
/// # Arguments
///
/// * `id` - An i32 that holds the ID of the transaction to delete.
///
/// # Returns
///
/// Returns an empty Result upon successful execution.
pub fn delete(id: i32) -> Result<()> {
    // Establishing a connection with the database
    let conn = initialize_database().unwrap();

    // Executing the SQL query to delete the transaction
    conn.execute("DELETE FROM transactions WHERE id = ?1", params![id])?;

    // Returning an Ok result upon successful deletion
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() -> Result<()> {
        let id = create(100.0, String::from("Test Transaction"), String::from("2023-06-15"), 1)?;
        assert!(id > 0);
        Ok(())
    }

    #[test]
    fn test_get_all() -> Result<()> {
        let transactions = get_all(1)?;
        assert!(transactions.len() > 0);
        Ok(())
    }

    #[test]
    fn test_delete() -> Result<()> {
        delete(1)?;
        let transactions = get_all(1)?;
        assert!(transactions.iter().find(|t| t.id == 1).is_none());
        Ok(())
    }
}
