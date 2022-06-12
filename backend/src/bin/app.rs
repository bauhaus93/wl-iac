extern crate chrono;
#[macro_use]
extern crate log;
extern crate dotenv;
extern crate log4rs;

extern crate wishlist;

use log::{LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use mongodb::Client;

#[tokio::main]
async fn main() {
    if init_logger() == false {
        return;
    }
    match dotenv::dotenv() {
        Ok(_) => {
            info!("Loaded dotenv!");
        }
        Err(e) => {
            warn!("Could not load dotenv: {}", e);
        }
    }

    let server_addr = match env::var("BACKEND_ADDRESS") {
        Ok(server_addr) => server_addr,
        Err(_) => {
            info!("No BACKEND_ADDRESS supplied, using default");
            String::from("0.0.0.0:8080")
        }
    };

    let socket_addr: SocketAddr = match server_addr.parse() {
        Ok(a) => a,
        Err(e) => {
            error!("Could not parse server addr '{}': {}", server_addr, e);
            return;
        }
    };

    info!("Server address: {}", server_addr);

    let mongo_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            error!("DATABASE_URL for MongoDB not set");
            return; 
        }
    };
    let mongo_client = match Client::with_uri_str(&mongo_url).await {
        Ok(c) => Arc::new(c),
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    let routes = match wishlist::create_routes(mongo_client).await {
        Ok(r) => r,
        Err(e) => {
            error!("Could not create routes: {}", e);
            return;
        }
    };
    info!("Created server routes");

    warp::serve(routes).run(socket_addr).await;
}

fn init_logger() -> bool {
    let logfile = match FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} {h({l})} - {m}\n",
        )))
        .build("log/output.log")
    {
        Ok(lf) => lf,
        Err(e) => {
            println!("Could not create logging FileAppender: {}", e);
            return false;
        }
    };

    let config = match Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
    {
        Ok(c) => c,
        Err(e) => {
            println!("Could not create logging Config: {}", e);
            return false;
        }
    };

    if let Err(e) = log4rs::init_config(config) {
        println!("Could not init logging config: {}", e);
        return false;
    }
    true
}
