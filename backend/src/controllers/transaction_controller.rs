use actix_web::{web, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use chrono::Utc;
use super::super::models::transaction_model;
use serde_json::json;

#[derive(Serialize)]
pub struct Transaction {
    pub id: i32,
    pub amount: f64,
    pub description: String,
    pub date: String,
    pub budget_group_id: i32,
}

#[derive(Deserialize)]
pub struct TransactionData {
    amount: f64,
    description: String,
    budget_group_id: i32,
}

pub async fn add_transaction(req_body: web::Json<TransactionData>) -> impl Responder {
    let date = Utc::now().to_string();
    println!("hit /expense/group/id");
    println!("amount: {}", req_body.amount);
    println!("description: {}", req_body.description);
    match transaction_model::create(req_body.amount.clone(), req_body.description.clone(), date, req_body.budget_group_id.clone()) {
        Ok(id) => {
            HttpResponse::Ok().json(json!({"id": id}))
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("error: {:?}", e))
    }
    
}