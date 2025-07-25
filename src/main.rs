use crate::engine::Engine;

mod engine;
mod service;

#[cfg(not(feature = "service"))]
fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).try_init().unwrap();
    // initailize the engine
    let engine = Engine::new();
}

#[cfg(feature = "service")]
#[tokio::main]
async fn main() {
    use std::env;
    use std::net::Ipv4Addr;
    use std::net::SocketAddr;
    env_logger::builder().filter_level(log::LevelFilter::Info).try_init().unwrap();

    let port: u16 = env::args().nth(1).unwrap().parse().unwrap();
    let addr = SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    service::AthenaServer::run_at(addr).await.unwrap();
}
