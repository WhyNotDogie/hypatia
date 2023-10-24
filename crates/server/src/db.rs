use std::env;

use common::GameInfo;
use parking_lot::Mutex;
use surrealdb::{Surreal, engine::remote::ws::{Client, Ws}};
use tokio::sync::OnceCell;

static DB: OnceCell<Mutex<Surreal<Client>>> = OnceCell::const_new();

pub async fn init() {
    let ep = env::var("DB_ENDPOINT").expect("Please specify database endpoint");
    DB.get_or_init(async || -> Mutex<Surreal<Client>> {
        Mutex::new(Surreal::new::<Ws>(ep).await.expect("Failed to connect to database"))
    }).await;
}

pub async fn get_info_from_game_id(id: u128) -> tide::Result<GameInfo> {
    // let db = DB.get().unwrap().lock();
    todo!()
}