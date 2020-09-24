use crate::models::{Status, CreateWorkerList, CreateWorker, ResultResponse};
use crate::db;
use deadpool_postgres::{Pool, Client};
use actix_web::{web, Responder, HttpResponse};
use std::io::ErrorKind::Other;


pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status { status: "OK".to_string() })
}

pub async fn get_list(db_pool: web::Data<Pool>) -> impl Responder {

    let client: Client =
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::get_list(&client).await;
    
    match result {
        Ok(workers) => HttpResponse::Ok().json(workers),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {

    let client: Client =
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::get_items(&client, path.0).await;
    
    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn create_list(db_pool: web::Data<Pool>, json: web::Json<CreateWorkerList>) -> impl Responder {

    let client: Client =
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::create_list(&client, json.category.clone()).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }

}

pub async fn create_item(db_pool: web::Data<Pool>, json: web::Json<CreateWorker>) -> impl Responder {

    let client: Client =
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::create_item(&client, json.bio.clone(), json.list_id.clone()).await;
    
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn check_item(db_pool: web::Data<Pool>, path: web::Path<(i32,i32)>) -> impl Responder {

    let client: Client =
        db_pool.get().await.expect("Error connecting to the database");
    
    let result = db::check_item(&client, path.0, path.1).await;    

    match result {
        Ok(()) => HttpResponse::Ok().json(ResultResponse{success: true}),
        Err(ref e) if e.kind() == Other => HttpResponse::Ok().json(ResultResponse{success: false}),
        Err(_) => HttpResponse::InternalServerError().into()
    }    
}
