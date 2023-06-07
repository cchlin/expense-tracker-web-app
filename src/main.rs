// mod controllers;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde::Serialize;
// use controllers::{transaction};

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
            // .service(web::scope("transaction")
                            // .route("", web::get().to(transaction::get_transaction)))
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

async fn index(template_enging: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = TemplateData {
        body: String::from("Hello World!"),
    };
    let body = template_enging.render("layout", &data).unwrap();
    HttpResponse::Ok().body(body)
}
