use crate::hashchain::mempool::Mempool;
use crate::server::handler;
use crate::server::security::keystore::KeyManager;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use std::sync::{Arc, Mutex};

pub async fn start(endpoint: String, mempool: Arc<Mutex<Mempool>>) -> std::io::Result<()> {
    println!("Started server at: {}", &endpoint);
    HttpServer::new(move || {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .data(KeyManager::restore())
            .data(mempool.clone())
            .service(web::resource("/status").route(web::get().to(handler::status)))
            .service(
                web::resource("/add_to_mempool")
                    .route(web::post().to(handler::add_to_mempool))
                    .guard(guard::fn_guard(|req| {
                        req.headers().contains_key("x-api-key")
                    }))
                    .to(|| HttpResponse::MethodNotAllowed()),
            )
            .service(
                web::resource("/get_history")
                    .route(web::get().to(handler::get_history))
            )
    })
    .bind(endpoint)?
    .run()
    .await
}
