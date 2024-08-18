use awc::Client;

use actix_web::{web, HttpResponse};
use anyhow::Result;

use actix_proxy::{IntoHttpResponse, SendRequestError};

pub async fn proxy(
    path: web::Path<(String,)>,
    client: web::Data<Client>,
) -> Result<HttpResponse, SendRequestError> {
    println!("byebyebye hi hi hi");
    let (url,) = path.into_inner();
    let url = format!("mockserver.com/{url}");
    Ok(client.get(&url).send().await?.into_http_response())
}
