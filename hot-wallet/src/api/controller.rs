pub mod model;
use actix_web::{web, get, post, HttpResponse, Responder};
use model::Wallet;
use mongodb::{Client};


const DB_NAME: &str = "test";
const COLL_NAME: &str = "wallets";

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello to my warm wallet!")
}

#[post("/create-wallet")]
pub async fn create_wallet(client: web::Data<Client>, form: web::Form<Wallet>) -> impl Responder {
  let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("wallet created with ID 345678909876543456"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/import-wallet")]
pub async fn import_wallet(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

#[actix_web::delete("/delete-wallet")]
pub async fn delete_wallet(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

#[post("/add-warm-wallet")]
pub async fn add_warm_wallet(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

#[get("/get-transaction")]
pub async fn get_transaction() -> impl Responder {
  HttpResponse::Ok().body("transactions")
}

#[get("/get-balance")]
pub async fn get_balance() -> impl Responder {
  HttpResponse::Ok().body("balance")
}

#[post("/adjust-balance-limit")]
pub async fn adjust_balance_limit(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

#[post("/receive-bitcoin")]
pub async fn receive_bitcoin(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

#[post("/send-bitcoin")]
pub async fn send_bitcoin(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}
