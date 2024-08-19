use url::Url;

use actix_web::{http::Method, web, HttpResponse};
use anyhow::Result;

use actix_proxy::{IntoHttpResponse, SendRequestError};

pub struct AppData {
    proxy_client: awc::Client,
    proxy_url: Url,
}
impl AppData {
    pub fn new(proxy_url: Url) -> Self {
        AppData {
            proxy_client: awc::Client::default(),
            proxy_url,
        }
    }
}

pub async fn proxy(
    method: Method,
    path: web::Path<(String,)>,
    client: web::Data<AppData>,
) -> Result<HttpResponse, SendRequestError> {
    let (url,) = path.into_inner();
    let url = client.proxy_url.join(&url).unwrap().to_string(); // TD: get rid of unwrap()
    Ok(client
        .proxy_client
        .request(method, &url)
        .send()
        .await?
        .into_http_response())
}
