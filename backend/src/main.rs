mod controllers;
mod models;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use controllers::{budget_group_controller, transaction_controller};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .route("/", web::get().to(index))
            .route(
                "/expense",
                web::get().to(budget_group_controller::get_budget_groups),
            )
            .route(
                "/expense/add-group",
                web::post().to(budget_group_controller::add_group),
            )
            .route(
                "/expense/group/{id}",
                web::delete().to(budget_group_controller::delete_group),
            )
            .route(
                "/expense/group",
                web::post().to(transaction_controller::add_transaction),
            )
            .route(
                "/expense/group/{id}",
                web::get().to(transaction_controller::get_transactions),
            )
            .route(
                "/expense/transaction",
                web::delete().to(transaction_controller::delete_transaction),
            )
            .route(
                "/expense/{id}",
                web::get().to(budget_group_controller::get_one_group),
            )
    })
    .bind(("127.0.0.1", 5001))?
    .run();

    println!("Server running on http://localhost:5001");

    server.await
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Budget Tracker App")
}
