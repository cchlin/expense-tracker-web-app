// use crate::models::connect;
use super::super::controllers::budget_group_controller::Group;
use super::connect::initialize_database;
use rusqlite::{params, Result};

/// Inserts a new budget group into the database.
///
/// # Arguments
///
/// * `name` - A string that holds the name of the budget group.
/// * `budget_amount` - A floating point that holds the budget amount for the group.
///
/// # Returns
///
/// Returns an i32 as the ID of the new budget group.
pub fn create(name: String, budget_amount: f64) -> Result<i32> {
    // Establishing a connection with the database
    let conn = initialize_database().unwrap();

    // Executing the SQL query to insert the new budget group
    conn.execute(
        "INSERT INTO budget_groups (name, budget_amount, remaining_budget) VALUES(?1, ?2, ?3)",
        params![name, budget_amount, budget_amount],
    )?;

    // Getting the ID of the last inserted group and returning it
    let id = conn.last_insert_rowid() as i32;
    Ok(id)
}

/// Retrieves all budget groups from the database.
///
/// # Returns
///
/// Returns a Vector of `Group` objects.
pub fn get_all() -> Result<Vec<Group>> {
    // Establishing a connection with the database
    let conn = initialize_database().unwrap();

    // Preparing the SQL statement
    let mut stmt = conn.prepare("SELECT * FROM budget_groups")?;

    // Executing the query and mapping the results to a list of groups
    let group_iter = stmt.query_map([], |row| {
        Ok(Group {
            id: row.get(0)?,
            name: row.get(1)?,
            budget_amount: row.get(2)?,
            remaining_budget: row.get(3)?,
        })
    })?;

    // Collecting the groups into a vector
    let mut groups = Vec::new();
    for group in group_iter {
        groups.push(group?);
    }

    // Returning the vector of groups
    Ok(groups)
}

/// Deletes a specific budget group and its associated transactions from the database.
///
/// # Arguments
///
/// * `id` - An i32 that holds the ID of the budget group to delete.
///
/// # Returns
///
/// Returns an empty Result upon successful execution.
pub fn delete(id: i32) -> Result<()> {
    // Establishing a connection with the database
    let conn = initialize_database().unwrap();

    // Executing the SQL query to delete the budget group
    conn.execute("DELETE FROM budget_groups WHERE id = ?1", params![id])?;

    // Executing the SQL query to delete all transactions associated with the budget group
    conn.execute(
        "DELETE FROM transactions WHERE budget_group_id = ?1",
        params![id],
    )?;

    // Returning an Ok result upon successful deletion
    Ok(())
}

/// Decreases the remaining budget of a specific budget group.
///
/// # Arguments
///
/// * `id` - An i32 that holds the ID of the budget group.
/// * `amount_to_deduct` - A floating point that holds the amount to deduct.
///
/// # Returns
///
/// Returns an empty Result upon successful execution.
pub fn minus_remaining(id: i32, amount_to_deduct: f64) -> Result<()> {
    // Establishing a connection with the database
    let conn = initialize_database().unwrap();

    // Executing the SQL query to reduce the remaining budget
    conn.execute(
        "UPDATE budget_groups SET remaining_budget = remaining_budget - ?1 WHERE id = ?2",
        params![amount_to_deduct, id],
    )?;

    // Returning an Ok result upon successful deduction
    Ok(())
}

/// Increases the remaining budget of a specific budget group.
///
/// # Arguments
///
/// * `id` - An i32 that holds the ID of the budget group.
/// * `amount_to_add` - A floating point that holds the amount to add.
///
/// # Returns
///
/// Returns an empty Result upon successful execution.
pub fn plus_remaining(id: i32, amount_to_add: f64) -> Result<()> {
    // Establishing a connection with the database
    let conn = initialize_database().unwrap();

    // Executing the SQL query to increase the remaining budget
    conn.execute(
        "UPDATE budget_groups SET remaining_budget = remaining_budget + ?1 WHERE id = ?2",
        params![amount_to_add, id],
    )?;

    // Returning an Ok result upon successful addition
    Ok(())
}

/// Retrieves a specific budget group from the database.
///
/// # Arguments
///
/// * `id` - An i32 that holds the ID of the budget group.
///
/// # Returns
///
/// Returns a `Group` object.
pub fn get_one(id: i32) -> Result<Group> {
    // Establishing a connection with the database
    let conn = initialize_database().unwrap();

    // Executing the SQL query to get a single group and returning the result
    let group: Group = conn.query_row(
        "SELECT * FROM budget_groups WHERE id = ?1",
        params![id],
        |row| {
            Ok(Group {
                id: row.get(0)?,
                name: row.get(1)?,
                budget_amount: row.get(2)?,
                remaining_budget: row.get(3)?,
            })
        },
    )?;

    Ok(group)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() -> Result<()> {
        let id = create(String::from("Test Group"), 1000.0)?;
        assert!(id > 0);
        Ok(())
    }

    #[test]
    fn test_get_all() -> Result<()> {
        let groups = get_all()?;
        assert!(groups.len() > 0);
        Ok(())
    }

    #[test]
    fn test_delete() -> Result<()> {
        delete(1)?;
        let groups = get_all()?;
        assert!(groups.iter().find(|g| g.id == 1).is_none());
        Ok(())
    }
}
