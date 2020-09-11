use crate::hashchain::mempool::Mempool;
use crate::server::is_valid;
use crate::server::security::keystore::KeyManager;
use crate::storage::read;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn status() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(format!("OK")))
}

pub async fn add_to_mempool(
    data: Option<String>,
    req: HttpRequest,
    store: web::Data<KeyManager>,
    mempool: web::Data<Arc<Mutex<Mempool>>>,
) -> Result<HttpResponse, Error> {
    let hash = store.keystore.api_key.clone();
    if is_valid(
        req.headers().get("x-api-key").unwrap().to_str().unwrap(),
        hash.clone(),
    ) {
        println!(
            "POST /add_to_mempool -- {:?} -- authorized request",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        match data {
            Some(data) => {
                mempool
                    .lock()
                    .expect("couldn't lock mempool")
                    .add(data.to_string());

                Ok(HttpResponse::Ok().json(data))
            }
            None => Ok(HttpResponse::Ok().json(format!("No thing!"))),
        }
    } else {
        Ok(HttpResponse::Unauthorized().json("Unauthorized"))
    }
}

pub async fn get_history() -> Result<HttpResponse, Error> {
    println!(
        "GET /get_history -- {:?} -- public endpoint",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    let history = read::all_blocks().unwrap();
    Ok(HttpResponse::Ok().json(history))
}
