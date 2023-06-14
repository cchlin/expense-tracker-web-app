use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Transaction {
    pub id: i32,
    pub amount: f64,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub budget_group_id: i32,
}