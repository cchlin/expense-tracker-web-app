use serde::Serialize;

#[derive(Serialize)]
pub struct Group {
    id: i32,
    name: String,
    budget_amount: f64,
    remaining_budget: f64,
}

pub async fn get_budget_groups() -> Vec<Group> {
    vec![
        Group {
            id: 1,
            name: String::from("Groceries"),
            budget_amount: 200.00,
            remaining_budget: 50.00,
        },
        Group {
            id: 2,
            name: String::from("Entertainment"),
            budget_amount: 100.00,
            remaining_budget: 30.00,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_get_budget_groups() {
        let group = get_budget_groups().await.unwrap();
        assert_eq!(group.id, 1);
        assert_eq!(group.name, "Groceries");
        assert_eq!(group.budget_amount, 200.00);
        assert_eq!(group.remaining_budget, 50.00);
    }
}
