use crate::routes::{health_check, subscribe};
use actix_web::{
    dev::Server,
    web::{get, post, Data},
    App, HttpServer,
};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    println!(":: RUNNING ::");
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", get().to(health_check))
            .route("/subscriptions", post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
