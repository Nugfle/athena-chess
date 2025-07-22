use crate::engine::Engine;

mod engine;
mod service;

#[cfg(not(feature = "service"))]
fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).try_init().unwrap();
    let engine = Engine::new();
}

#[cfg(feature = "service")]
#[tokio::main]
async fn main() {
    use std::env;
    use std::net::SocketAddr;
    let addr: SocketAddr = env::args().nth(1).unwrap().parse().unwrap();

    service::AthenaServer::run_at(addr).await.unwrap();
}
