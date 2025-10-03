use athena_core::game::Game;
use error::ServiceError;
use log::info;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

mod error;

#[tokio::main]
async fn main() {
    use std::env;
    use std::net::Ipv4Addr;
    use std::net::SocketAddr;

    env_logger::builder().filter_level(log::LevelFilter::Info).try_init().unwrap();

    let port: u16 = env::args().nth(1).unwrap().parse().unwrap();
    let addr = SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    AthenaServer::run_at(addr).await.unwrap();
}

pub struct AthenaServer {}

impl AthenaServer {
    pub async fn run_at(addr: impl ToSocketAddrs) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(addr).await?;
        while let Ok((conn, ip)) = listener.accept().await {
            tokio::spawn(async move {
                let mut s = AthenaService::new();
                s.run_service(conn, ip).await.unwrap();
            });
        }
        Ok(())
    }
}

pub struct AthenaService {
    game: Game,
}

impl AthenaService {
    pub fn new() -> Self {
        Self { game: Game::init() }
    }

    pub async fn run_service(&mut self, conn: TcpStream, addr: SocketAddr) -> Result<(), ServiceError> {
        info!("got connection from: {}", addr);
        todo!("implement simple protocol for chess engine interaction");
        Ok(())
    }
}
