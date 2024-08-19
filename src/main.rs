use actix_web::web;
use url::Url;

use actix_web::{App, HttpServer};

mod config;
mod proxy;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let proxy_url = Url::parse("http://localhost-machine:123")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    HttpServer::new(move || {
        App::new()
            .service(web::resource("/proxy/{url:.*}").route(web::to(proxy::proxy)))
            .app_data(web::Data::new(proxy::AppData::new(proxy_url.clone())))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
