use actix_web::HttpResponse;

// #[derive(Deserialize)]
// struct FormData {
//     email: String,
//     name: String,
// }

pub async fn health_check() -> HttpResponse {
    println!(":: HEALTH CHECK ::");
    HttpResponse::Ok().finish()
}

// pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| {
//         App::new().route("/health_check", web::get().to(health_check))
//         // .route("/subscriptions", web::post().to(subscribe))
//     })
//     .listen(listener)?
//     .run();

//     Ok(server)
// }

// async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
//     HttpResponse::Ok().finish()
// }
