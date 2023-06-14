// use crate::models::connect;
use super::super::controllers::budget_group_controller::Group;
use super::connect::initialize_database;
use rusqlite::{params, Result};

// create a new budget group
pub fn create(name: String, budget_amount: f64) -> Result<i32> {
    let conn = initialize_database().unwrap();
    conn.execute(
        "INSERT INTO budget_groups (name, budget_amount, remaining_budget) VALUES(?1, ?2, ?3)",
        params![name, budget_amount, budget_amount],
    )?;

    // Get the id of the last inserted row
    let id = conn.last_insert_rowid() as i32;
    Ok(id)
}

// get all groups in the db
pub fn get_all() -> Result<Vec<Group>> {
    let conn = initialize_database().unwrap();

    let mut stmt = conn.prepare("SELECT * FROM budget_groups")?;

    let group_iter = stmt.query_map([], |row| {
        Ok(Group {
            id: row.get(0)?,
            name: row.get(1)?,
            budget_amount: row.get(2)?,
            remaining_budget: row.get(3)?,
        })
    })?;

    let mut groups = Vec::new();

    for group in group_iter {
        groups.push(group?);
    }

    Ok(groups)
}

pub fn delete(id: i32) -> Result<()> {
    let conn = initialize_database().unwrap();

    conn.execute("DELETE FROM budget_groups WHERE id = ?1", params![id])?;

    conn.execute(
        "DELETE FROM transactions WHERE budget_group_id = ?1",
        params![id],
    )?;

    Ok(())
}

pub fn minus_remaining(id: i32, amount_to_deduct: f64) -> Result<()> {
    let conn = initialize_database().unwrap();

    conn.execute(
        "UPDATE budget_groups SET remaining_budget = remaining_budget - ?1 WHERE id = ?2",
        params![amount_to_deduct, id],
    )?;

    Ok(())
}

pub fn plus_remaining(id: i32, amount_to_add: f64) -> Result<()> {
    let conn = initialize_database().unwrap();

    conn.execute(
        "UPDATE budget_groups SET remaining_budget = remaining_budget + ?1 WHERE id = ?2",
        params![amount_to_add, id],
    )?;

    Ok(())
}

pub fn get_one(id: i32) -> Result<Group> {
    let conn = initialize_database().unwrap();

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
