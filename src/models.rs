use serde::{Serialize,Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="worker_list")]
pub struct WorkerList {
    pub id: i32,
    pub category: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="worker")]
pub struct Worker {
    pub id: i32,
    pub bio: String,
    pub tip_method: String,
    pub checked: bool,
    pub list_id: i32 
}

#[derive(Deserialize)]
pub struct CreateWorkerList {
    pub category: String 
}

#[derive(Deserialize)]
pub struct CreateWorker {
    pub bio: String,
    pub list_id: i32
}

#[derive(Serialize)]
pub struct ResultResponse {
    pub success: bool
}