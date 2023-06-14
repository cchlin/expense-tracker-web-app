use super::super::models::budget_group_model;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Struct representing a Group object
#[derive(Serialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub budget_amount: f64,
    pub remaining_budget: f64,
}

/// Returns all budget groups
pub async fn get_budget_groups() -> impl Responder {
    match budget_group_model::get_all() {
        Ok(groups) => HttpResponse::Ok().json(&groups),
        Err(e) => HttpResponse::InternalServerError().body(format!("error: {}", e)),
    }
}

/// Struct to receive data for a new group from a form
#[derive(Deserialize)]
pub struct FormData {
    name: String,
    budget_amount: f64,
}

/// Adds a new budget group
pub async fn add_group(req_body: web::Json<FormData>) -> impl Responder {
    match budget_group_model::create(req_body.name.clone(), req_body.budget_amount) {
        Ok(id) => HttpResponse::Ok().json(json!({ "id": id })),
        Err(e) => {
            println!("error: {:?}", e);
            HttpResponse::InternalServerError().body(format!("error: {:?}", e))
        }
    }
}

/// Deletes a budget group given its ID
pub async fn delete_group(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match budget_group_model::delete(id) {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
        Err(e) => HttpResponse::InternalServerError().body(format!("error: {:?}", e)),
    }
}

/// Returns one budget group given its ID
pub async fn get_one_group(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match budget_group_model::get_one(id) {
        Ok(group) => HttpResponse::Ok().json(&group),
        Err(e) => HttpResponse::InternalServerError().body(format!("error: {:?}", e)),
    }
}