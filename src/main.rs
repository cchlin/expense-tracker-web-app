mod controllers;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use controllers::budget_group_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "./src/templates")
        .expect("Error setting up handlebars");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(handlebars.clone()))
            .route("/", web::get().to(index))
            .route("/expense", web::get().to(get_budget_groups))
    })
    .bind(("127.0.0.1", 5001))?
    .run();

    println!("Server running on http://localhost:5001");

    server.await
}

#[derive(Serialize)]
struct TemplateData {
    body: String,
}

pub async fn index(template_enging: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = TemplateData { 
        body: String::from("Hello World!"),
    };
    let index_content = template_enging.render("index", &data).unwrap();

    let content = TemplateData {
        body: index_content,
    };
    let body = template_enging.render("layout", &content).unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn get_budget_groups(template_engine: web::Data<Handlebars<'_>>) -> impl Responder {
    match budget_group_controller::get_budget_groups().await {
        Some(group) => {
            let data = json!({ "group": group });

            let group_content = template_engine.render("expense/budget_group", &data).unwrap();

            let layout_data = json!({ "body": group_content });

            let body = template_engine.render("layout", &layout_data).unwrap();

            HttpResponse::Ok().body(body)
        },
        None => {
            HttpResponse::NotFound().body("Group not found")
        }
    }
}

