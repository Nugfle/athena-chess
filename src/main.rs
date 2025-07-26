pub mod game;

#[cfg(not(feature = "service"))]
fn main() {
    use crate::game::{File, Game, Move, Piece, Rank, Square};

    env_logger::builder().filter_level(log::LevelFilter::Info).try_init().unwrap();
    // initailize the engine
    let mut game = Game::init();
    game.execute_move(Move::new(
        Piece::Pawn { en_pasent: false },
        Square::from_rank_file(Rank::Two, File::E),
        Square::from_rank_file(Rank::Four, File::E),
        None,
    ))
    .unwrap();
}

#[cfg(feature = "service")]
#[tokio::main]
async fn main() {
    use std::env;
    use std::net::Ipv4Addr;
    use std::net::SocketAddr;
    mod service;

    env_logger::builder().filter_level(log::LevelFilter::Info).try_init().unwrap();

    let port: u16 = env::args().nth(1).unwrap().parse().unwrap();
    let addr = SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    service::AthenaServer::run_at(addr).await.unwrap();
}
