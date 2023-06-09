use rusqlite::{Connection, Result};

pub fn initialize_database() -> Result<Connection> {    
    // Open a new connection to a SQLite database. If a database
    // does not exist at the path, one is created.
    let connection = Connection::open("expense_tracker_database.db")?;

    // creates the transactions table if it does not exist
    connection.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY,
            amount REAL NOT NULL,
            description TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            budget_group_id INTEGER NOT NULL,
            FOREIGN KEY(budget_group_id) REFERENCES budget_groups(id)
        )",
        (),
    )?;

    // creates the budget_groups table if it does not exist
    connection.execute(
        "CREATE TABLE IF NOT EXISTS budget_groups (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            budget_amount REAL NOT NULL,
            remaining_budget REAL NOT NULL
        )",
        (),
    )?;

    // return the connection
    Ok(connection)
}