// use crate::models::connect;
use rusqlite::{params, Result};
use super::connect::initialize_database;
use super::super::controllers::budget_group_controller::Group;

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

    conn.execute(
        "DELETE FROM budget_groups WHERE id = ?1", params![id])?;

    Ok(())
}