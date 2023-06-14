mod controllers;
mod models;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use controllers::budget_group_controller;
use serde::Deserialize;
use serde_json::json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .route("/", web::get().to(index))
            .route("/expense", web::get().to(get_budget_groups))
            .route("/expense/add-group", web::post().to(add_group))
    })
    .bind(("127.0.0.1", 5001))?
    .run();

    println!("Server running on http://localhost:5001");

    server.await
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Budget Tracker App")
}

pub async fn get_budget_groups() -> impl Responder {
    println!("hit /expense route");
    let groups = budget_group_controller::get_budget_groups().await;
    if groups.is_empty() {
        HttpResponse::NotFound().body("No groups found")
    } else {
        HttpResponse::Ok().json(&groups)
    }
}

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    budget_amount: f64,
}

// #[post("/expense/add_group")]
pub async fn add_group(req_body: web::Json<FormData>) -> impl Responder {
    println!("hit /expense/add-group");
    println!(
        "Name: {}, Budget: {}",
        req_body.name, req_body.budget_amount
    );
    let data = FormData {
        name: req_body.name.clone(),
        budget_amount: req_body.budget_amount,
    };
    match budget_group_controller::add_group(data).await {
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

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::test;
    use actix_web::{web, App};
    use serde_json::Value;

    #[actix_rt::test]
    async fn test_get_budget_groups() {
        let app = actix_web::test::init_service(
            App::new().route("/expense", web::get().to(super::get_budget_groups)),
        )
        .await;

        let req = test::TestRequest::get().uri("/expense").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let body_json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body_json["id"], 1);
        assert_eq!(body_json["name"], "Groceries");
        assert_eq!(body_json["budget_amount"], 200.00);
        assert_eq!(body_json["remaining_budget"], 50.00);
    }
}
