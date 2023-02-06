use std::net::TcpListener;

use actix_web::{HttpServer, App, HttpResponse, web::{self, Bytes}, dev::{self, ResourcePath, ServiceResponse, Server}, Error, body::BoxBody, FromRequest};
use openssl::ssl::{SslFiletype, SslMethod, SslAcceptor};
use reqwest::Version;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
enum Logging {
  #[serde(rename = "stdio")]
  Stdio,
  File(String)
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
struct Tls {
  enabled: bool,
  key: String,
  cert: String
}

impl Default for Tls {
  fn default() -> Self {
    Self { enabled: false, key: "key.pem".into(), cert: "cert.pem".into() }
  }
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
struct Proxy {
  max_header: Option<usize>,
  max_body: Option<usize>,
  buffers: usize,
  pool_limit: Option<usize>,
  keep_alive: usize,
  timeout: usize,
}

impl Default for Proxy {
  fn default() -> Self {
    Self {
      max_header: Some(1024),
      max_body: Some(1024),
      buffers: 4096,
      pool_limit: Some(64),
      keep_alive: 3600,
      timeout: 60
    }
  }
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct AppConf {
  tls: Tls,
  proxy: Proxy,
  logging: Logging
}

impl Default for AppConf {
  fn default() -> Self {
    Self {
      tls: Tls::default(),
      proxy: Proxy::default(),
      logging: Logging::Stdio
    }
  }
}


async fn proxy(req: dev::ServiceRequest) -> Result<dev::ServiceResponse, Error> {
  let uri = req.uri().to_string();
  
  let method = req.method().to_owned();

  let url = reqwest::Url::parse(
    &format!("http://127.0.0.1:8081{}", uri.path())
  ).unwrap();
  
  let p_res = reqwest::Client::builder()
    .build().unwrap()
    .request(method, url);
  let p_res = if let Ok(body) = Bytes::extract(req.request()).await {
    p_res.body(body)
      .send().await.unwrap()
  } else {
    p_res.send().await.unwrap()
  };

  let code = p_res.status();
  let bytes = p_res.bytes().await.unwrap();

  let res = 
    HttpResponse::new(code)
      .set_body(BoxBody::new(bytes));
  
  Ok( ServiceResponse::new(req.request().to_owned(), res) )
}

pub fn run(listener: TcpListener, app: AppConf) -> Result<Server, Box<dyn std::error::Error>> {
  let proxy = HttpServer::new( || {
    App::new().service(
      web::service("{any}")
        .finish(proxy)
    )
  });

  if app.tls.enabled {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file(app.tls.key, SslFiletype::PEM)?;
    builder.set_certificate_chain_file(app.tls.cert)?;
    Ok(proxy.listen_openssl(listener, builder)?.run())
  }
  else {
    Ok(proxy.listen(listener)?.run())
  }
}