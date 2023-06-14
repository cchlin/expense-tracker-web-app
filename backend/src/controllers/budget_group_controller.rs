use super::super::models::budget_group_model;
use serde::{Serialize, Deserialize};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

#[derive(Serialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub budget_amount: f64,
    pub remaining_budget: f64,
}

// pub async fn get_budget_groups() -> Vec<Group> {
//     match budget_group_model::get_all() {
//         Ok(groups) => {
//             groups
//         },
//         Err(_) => Vec::new(),
//     }
// }

pub async fn get_budget_groups() -> impl Responder {
    match budget_group_model::get_all() {
        Ok(groups) => {
            HttpResponse::Ok().json(&groups)
        },
        Err(_e) => {
            HttpResponse::Ok().body(Vec::new())
        }
    }
}

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    budget_amount: f64,
}

pub async fn add_group(req_body: web::Json<FormData>) -> impl Responder {
    match budget_group_model::create(req_body.name.clone(), req_body.budget_amount.clone()) {
        Ok(id) => {
            println!("id: {}", id);
            HttpResponse::Ok().json(json!({"id": id}))
        }
        Err(e) => {
            println!("error: {:?}", e);
            HttpResponse::InternalServerError().body(format!("error: {:?}", e))
        }
    }
}

pub async fn delete_group(path: web::Path<i32>) -> impl Responder {
    println!("hit /expense/group delete");
    let id = path.into_inner();
    match budget_group_model::delete(id) {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
        Err(e) => HttpResponse::InternalServerError().body(format!("error: {:?}", e))
    }
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
