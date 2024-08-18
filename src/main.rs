use actix_web::web;

use actix_web::{App, HttpServer};

mod config;
mod proxy;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("bye");
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/proxy/{url:.*}").route(web::to(proxy::proxy)))
            .app_data(web::Data::new(awc::Client::default()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
