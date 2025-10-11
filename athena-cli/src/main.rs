fn main() {
    use athena_core::game::{File, Game, Move, Piece, Rank, Square};
    env_logger::builder().filter_level(log::LevelFilter::Info).try_init().unwrap();
    // initailize the engine
    let mut game = Game::init();
    game.execute_move(Move::new(
        Piece::Pawn,
        Square::from_rank_file(Rank::Two, File::E),
        Square::from_rank_file(Rank::Four, File::E),
        None,
    ))
}
