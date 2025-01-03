pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;

// pub trait Log: Sync + Send {
//     fn enabled(&self, metadata: &Metadata) -> bool;
//     fn log(&self, record: &Record);

//     fn flush(&self);
// }

// use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
// use serde::Deserialize;
// use std::net::TcpListener;

// #[derive(Deserialize)]
// struct FormData {
//     email: String,
//     name: String,
// }

// async fn health_check() -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| {
//         App::new()
//             .route("/health", web::get().to(health_check))
//             .route("/subscriptions", web::post().to(subscribe))
//     })
//     .listen(listener)?
//     .run();

//     Ok(server)
// }

// async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
//     HttpResponse::Ok().finish()
// }
