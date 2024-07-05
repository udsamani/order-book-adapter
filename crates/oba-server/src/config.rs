use std::env;
use tokio::sync::OnceCell;

use dotenvy::dotenv;

#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug)]
pub struct Config {
    server: ServerConfig,
}


impl Config {
    pub fn server_host(&self) -> &str {
        &self.server.host
    }

    pub fn server_port(&self) -> u16 {
        self.server.port
    }
}


pub static CONFIG: OnceCell<Config>  = OnceCell::const_new();

async fn init_config() -> Config {
    dotenv().ok();

    let server_config = ServerConfig{
        host: env::var("OBA_HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
        port: env::var("OBA_PORT").unwrap_or_else(|_| String::from("3000")).parse::<u16>().unwrap(),
    };

    Config {
        server: server_config,
    }
}



pub async fn config() -> &'static Config {
    CONFIG.get_or_init(init_config).await
}
