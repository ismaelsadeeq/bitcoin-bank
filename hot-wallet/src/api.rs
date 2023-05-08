mod controller;

#[cfg(test)]
mod test;


use actix_web::{web,App, HttpServer};
use crate::api::controller::*;
use mongodb::{Client};
use dotenv::dotenv;

pub async fn run() -> std::io::Result<()> {
  dotenv().ok();

  const PORT:u16 = 8080;

  let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

  let client = Client::with_uri_str(uri).await.expect("failed to connect");
  HttpServer::new(move|| {
    App::new()
        .app_data(web::Data::new(client.clone()))
        .service(hello)
        .service(create_wallet)
        .service(import_wallet)
        .service(add_warm_wallet)
        .service(get_transaction)
        .service(get_balance)
        .service(adjust_balance_limit)
        .service(receive_bitcoin)
        .service(send_bitcoin)

  })
  .bind(("127.0.0.1",PORT))?
  .run()
  .await
}