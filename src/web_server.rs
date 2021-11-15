use std::sync::{Arc, Mutex};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware};
use serde::{Deserialize, Serialize};
use crate::block_chain::{Blockchain, Block};

struct AppState {
    blockchain: Blockchain
}

#[derive(Deserialize, Serialize)]
struct MineBlockResponse {
    message: String,
    index: usize,
    timestamp: String,
    proof: usize,
    previous_hash: String
}

#[derive(Deserialize, Serialize)]
struct GetChainResponse {
  chain: Vec<Block>,
  length: usize
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/mine_block")]
async fn mine_block(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let mut data = data.lock().unwrap();
    let previous_block = data.blockchain.get_previous_block();
    let previous_proof = previous_block.proof;
    let proof = data.blockchain.proof_of_work(previous_proof);
    let previous_hash = data.blockchain.hash(previous_block);
    let block = data.blockchain.create_block(proof, previous_hash);

    HttpResponse::Ok().json(MineBlockResponse {
        message: String::from("Congratulations, you just mined a block!"),
        index: block.index,
        timestamp: block.timestamp.clone(),
        proof: block.proof,
        previous_hash: block.previous_hash.clone()
    })
}

#[get("/get_chain")]
async fn get_chain(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
  let data = data.lock().unwrap();

  HttpResponse::Ok().json(GetChainResponse {
    chain: data.blockchain.chain.clone(),
    length: data.blockchain.chain.len()
  })
}

#[get("/is_valid")]
async fn is_valid(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
  let data = data.lock().unwrap();

  HttpResponse::Ok().body(if data.blockchain.is_chain_valid(&data.blockchain.chain) {
    "True"
  } else {
    "False"
  })
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();

    let app_state = Arc::new(Mutex::new(AppState {
        blockchain: Blockchain::new()
    }));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(app_state.clone())
            .service(hello)
            .service(get_chain)
            .service(mine_block)
            .service(is_valid)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}