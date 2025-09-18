mod config;
mod db;
mod routes;
mod models;
mod schema;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use config::AppConfig;
use db::{init_pool, run_migrations};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let cfg = AppConfig::from_env();
    let pool = init_pool(&cfg.database_url);

    run_migrations(&pool);

    println!("➡️  Server running at http://{}/", cfg.bind_addr());

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(actix_web::web::Data::new(pool.clone()))
            .configure(routes::init)
    })
    .bind(cfg.bind_addr())?
    .run()
    .await
}
