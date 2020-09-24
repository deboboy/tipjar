use crate::models::{Worker, WorkerList};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn get_list(client: &Client) -> Result<Vec<WorkerList>, io::Error> {

    let statement = client.prepare("select * from worker_list order by id desc").await.unwrap();

    let lists = client.query(&statement, &[])
        .await
        .expect("Error getting worker list")
        .iter()
        .map(|row | WorkerList::from_row_ref(row).unwrap())
        .collect::<Vec<WorkerList>>();
        
    Ok(lists)    
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<Worker>, io::Error> {
    let statement = client.prepare("select * from worker where list_id = $1 order by id").await.unwrap();

    let items = client.query(&statement, &[&list_id])
        .await
        .expect("Error getting workers")
        .iter()
        .map(|row| Worker::from_row_ref(row).unwrap())
        .collect::<Vec<Worker>>();

    Ok(items)
}

pub async fn create_list(client: &Client, category: String) -> Result<WorkerList, io::Error> {
    let statement = client.prepare("insert into worker_list (category) values ($1) returning id, category").await.unwrap();

    client.query(&statement, &[&category])
        .await
        .expect("Error creating worker list")
        .iter()
        .map(|row | WorkerList::from_row_ref(row).unwrap())
        .collect::<Vec<WorkerList>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating worker list"))
}

pub async fn create_item(client: &Client, bio: String, list_id: i32) -> Result<Worker, io::Error> {
    let statement = client.prepare("insert into worker (bio, list_id) values ($1) returning id, bio").await.unwrap();

    client.query(&statement, &[&bio, &list_id])
        .await
        .expect("Error creating worker")
        .iter()
        .map(|row | Worker::from_row_ref(row).unwrap())
        .collect::<Vec<Worker>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating worker"))
}

pub async fn check_item(client: &Client, list_id: i32, item_id: i32) -> Result<(), io::Error> {

    let statement = client.prepare("update worker set checked = true where list_id = $1 and id = $2 and checked = false").await.unwrap();

    let result = client.execute(&statement, &[&list_id, &item_id])
        .await
        .expect("Error checking worker");

    match result{
        ref updated if *updated == 1 => Ok(()),
            _=> Err(io::Error::new(io::ErrorKind::Other, "Failed to check list"))  
    }

}