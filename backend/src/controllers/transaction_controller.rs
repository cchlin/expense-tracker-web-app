use super::super::models::{budget_group_model, transaction_model};
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Struct representing a Transaction object
#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub amount: f64,
    pub description: String,
    pub date: String,
    pub budget_group_id: i32,
}

/// Struct to receive data for a new transaction from a form
#[derive(Deserialize)]
pub struct TransactionData {
    amount: f64,
    description: String,
    budget_group_id: i32,
}

/// Returns all transactions for a given budget group ID
pub async fn get_transactions(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match transaction_model::get_all(id) {
        Ok(transactions) => HttpResponse::Ok().json(&transactions),
        Err(e) => HttpResponse::InternalServerError().body(format!("error: {}", e)),
    }
}

/// Adds a new transaction for a given budget group ID
pub async fn add_transaction(req_body: web::Json<TransactionData>) -> impl Responder {
    let date = Utc::now().format("%Y-%m-%d %H:%M").to_string();
    match transaction_model::create(
        req_body.amount,
        req_body.description.clone(),
        date,
        req_body.budget_group_id,
    ) {
        Ok(id) => {
            // update the remaining budget in the group
            match budget_group_model::minus_remaining(req_body.budget_group_id, req_body.amount) {
                Ok(()) => HttpResponse::Ok().json(json!({ "id": id })),
                Err(e) => {
                    println!("Error updating remaining budget: {:?}", e);
                    HttpResponse::InternalServerError().body(format!("error: {:?}", e))
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError().body(format!("error: {:?}", e))
        }
    }
}

/// Deletes a transaction given its ID and updates the corresponding budget group's remaining budget
pub async fn delete_transaction(req_body: web::Json<Transaction>) -> impl Responder {
    let id = req_body.id;
    let amount = req_body.amount;
    let budget_group_id = req_body.budget_group_id;
    match transaction_model::delete(id) {
        Ok(_) => match budget_group_model::plus_remaining(budget_group_id, amount) {
            Ok(()) => HttpResponse::Ok().json(json!({"status": "success"})),
            Err(e) => {
                println!("Error updating remaining budget: {:?}", e);
                HttpResponse::InternalServerError().body(format!("error: {:?}", e))
            }
        },

        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError().body(format!("error: {:?}", e))
        }
    }
}
