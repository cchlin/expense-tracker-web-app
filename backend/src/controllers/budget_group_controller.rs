use super::super::models::budget_group_model;
use super::super::FormData;
use rusqlite::Result;
use serde::Serialize;

#[derive(Serialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub budget_amount: f64,
    pub remaining_budget: f64,
}

pub async fn get_budget_groups() -> Vec<Group> {
    match budget_group_model::get_all() {
        Ok(groups) => {
            groups
        },
        Err(_) => Vec::new(),
    }
}

pub async fn add_group(form_data: FormData) -> Result<i32> {
    budget_group_model::create(form_data.name, form_data.budget_amount)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[actix_rt::test]
//     async fn test_get_budget_groups() {
//         let group = get_budget_groups().await;
//         assert_eq!(group.id, 1);
//         assert_eq!(group.name, "Groceries");
//         assert_eq!(group.budget_amount, 200.00);
//         assert_eq!(group.remaining_budget, 50.00);
//     }
// }
