use super::controller::model;

use actix_web::{
  test::{call_and_read_body, init_service, TestRequest},
  web::Bytes,
};
use mongodb::Client;
use dotenv::dotenv;


use super::*;
const DB_NAME: &str = "test";
const COLL_NAME: &str = "wallets";

#[actix_web::test]
// #[ignore = "requires MongoDB instance running"]
async fn test() {
  dotenv().ok();

  let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

  let client = Client::with_uri_str(uri)
    .await.
    expect("failed to connect");

  // Clear any data currently in the wallet collection.
  client
      .database(DB_NAME)
      .collection::<model::Wallet>(COLL_NAME)
      .drop(None)
      .await
      .expect("drop collection should succeed");

  let app = init_service(
      App::new()
          .app_data(web::Data::new(client))
          // .service(hello)
          .service(create_wallet)
          // .service(import_wallet)
          // .service(add_warm_wallet)
          // .service(get_transaction)
          // .service(get_balance)
          // .service(adjust_balance_limit)
          // .service(receive_bitcoin)
          // .service(send_bitcoin)
  )
  .await;

  let wallet = model::Wallet {
      xpub_decs: "wpkh()".into(),
  };

  let req = TestRequest::post()
      .uri("/create-wallet")
      .set_form(&wallet)
      .to_request();

  let response = call_and_read_body(&app, req).await;
  println!("{:?}", response);
  assert_eq!(response, Bytes::from_static(b"wallet created with ID 345678909876543456"));

  // let req = TestRequest::get()
  //     .uri(&format!("/import-wallet/{}", &user.xpub_decs))
  //     .to_request();

  // let response: Wallet = call_and_read_body_json(&app, req).await;
  // assert_eq!(response, user);
}