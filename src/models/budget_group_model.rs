use rusqlite::{params, Connection, Result};
use crate::models::connect;

pub struct BudgetGroup {
    id: i32,
    name: String,
    budget_amount: f64,
    remaining_budget: f64,
}