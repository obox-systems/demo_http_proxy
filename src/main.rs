use std::net::TcpListener;

use config::{Config, File, FileFormat};
use http_stream_proxy::{run, AppConf};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let config = Config::builder()
    .add_source(File::new("config.toml", FileFormat::Toml).required(false))
    .build()
    .unwrap();

  let app: AppConf = config.try_deserialize().unwrap();

  let http = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind proxy on port 8080");

  _ = run(http, app)?.await;

  Ok(())
}
