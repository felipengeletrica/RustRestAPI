#[macro_use] extern crate log;
extern crate env_logger;
extern crate contract;
extern crate actix_web;

mod endpoints;
use endpoints::*;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    info!("Rust Actix Server running... http://localhost:8181/");
    HttpServer::new(|| App::new()
        .service(index)
        .service(list_edges)
        .service(insert_edge)
        .service(get_edge_by_id)
        .service(delete_edge_by_id)
    )
    .bind("127.0.0.1:8181")?
    .run()
    .await
}