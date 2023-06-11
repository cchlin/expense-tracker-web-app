mod controllers;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use controllers::budget_group_controller;
use serde_json::json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .route("/", web::get().to(index))
            .route("/expense", web::get().to(get_budget_groups))
    })
    .bind(("127.0.0.1", 5001))?
    .run();

    println!("Server running on http://localhost:5001");

    server.await
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Expense Tracker App")
}

pub async fn get_budget_groups() -> impl Responder {
    let groups = budget_group_controller::get_budget_groups().await;
    if groups.is_empty() {
        HttpResponse::NotFound().body("No groups found")
    } else {
        HttpResponse::Ok().json(&groups)
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
