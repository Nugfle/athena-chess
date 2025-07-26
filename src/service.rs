use crate::game::Game;
use error::ServiceError;
use log::info;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

mod error;

pub struct AthenaServer {}

impl AthenaServer {
    pub async fn run_at<T>(addr: T) -> Result<(), std::io::Error>
    where
        T: ToSocketAddrs,
    {
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
