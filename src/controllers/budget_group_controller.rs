use serde::Serialize;

#[derive(Serialize)]
pub struct Group {
    id: i32,
    name: String,
    budget_amount: f64,
    remaining_budget: f64,
}

pub async fn get_budget_groups() -> Option<Group> {

    let group = Group {
        id: 1,
        name: String::from("Groceries"),
        budget_amount: 200.00,
        remaining_budget: 50.00,
    };

    Some(group)

}