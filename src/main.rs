mod config;
mod models;
mod handlers;
mod db;

use actix_web::{HttpServer, web, App};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::*;

#[actix_rt::main]
async fn main () -> io::Result<()> {

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Starting server at http://{}:{}/", config.server.host, config.server.port); 

    HttpServer::new(move || {

        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/workers{_:/?}", web::get().to(get_list))
            .route("/workers{_:/?}", web::post().to(create_list))
            .route("/workers/{list_id}/items{_:/?}", web::post().to(create_item))
            .route("/workers/{list_id}/items{_:/?}", web::get().to(get_items))
            .route("/workers/{list_id}/items/{item_id}{_:/?}", web::put().to(check_item))

    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}