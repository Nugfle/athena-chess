use attack_tables::AttackTables;
use board::BitBoard;
use log::info;
pub use mask::BoardMask;
use std::sync::LazyLock;

pub use board::piece::{Color, Piece};
pub use board::square::*;
pub use chess_move::Move;

mod attack_tables;
mod board;
mod chess_move;
mod error;
mod evaluation;
mod mask;

static ATTACK_TABLES: LazyLock<AttackTables> = LazyLock::new(|| {
    let start = std::time::Instant::now();
    let at = AttackTables::create_tables();
    let took = start.elapsed().as_millis();
    info!("built attack tables, took {took} ms...");
    at
});

#[cfg(feature = "benchmark")]
pub fn create_tables() {
    AttackTables::create_tables();
}

#[derive(Debug, Clone)]
pub struct Game {
    board: BitBoard,
    moves: Vec<Move>,
    turn: Color,
}

impl Game {
    pub fn init() -> Self {
        _ = ATTACK_TABLES;
        Self {
            board: BitBoard::init(),
            moves: Vec::new(),
            turn: Color::White,
        }
    }

    pub fn execute_move(&mut self, mut mv: Move) {
        let from = mv.get_from();
        let to = mv.get_to();

        let (mut temp_p, temp_c) = self.board.remove_piece_from_square(from).expect("checked that from is Some");
        temp_p.make_moved();
        let takes = self.board.place_piece_on_square(temp_p, temp_c, to).map(|(taken, _)| taken);
        mv.set_takes(takes);

        self.moves.push(mv);
        self.turn = !self.turn;
    }

    pub fn get_available_moves(&self) -> Vec<Move> {
        let pieces: Vec<(Square, Piece)> = self
            .board
            .board
            .iter()
            .enumerate()
            .filter_map(|(i, item)| {
                item.and_then(|(pc, col)| {
                    if col == self.turn {
                        Some((Square::try_from(i).unwrap(), pc))
                    } else {
                        None
                    }
                })
            })
            .collect();

        pieces
            .iter()
            .map(|(sq, pc)| {
                let board_mask = match pc {
                    Piece::Rook { .. } => ATTACK_TABLES.get_attack_pattern_rook(*sq, self.board.occupancy),
                    Piece::Pawn => todo!(""),
                    Piece::King { .. } => todo!(""),
                    Piece::Knight => ATTACK_TABLES.get_attack_pattern_knight(*sq),
                    Piece::Queen => ATTACK_TABLES.get_attack_pattern_queen(*sq, self.board.occupancy),
                    Piece::Bishop => ATTACK_TABLES.get_attack_pattern_bishop(*sq, self.board.occupancy),
                };
                self.moves_from_mask_and_starting_square(&board_mask, *sq)
            })
            .flatten()
            .collect()
    }

    pub fn previous_move(&self) -> Option<&Move> {
        self.moves.last()
    }
    fn moves_from_mask_and_starting_square(&self, board_mask: &BoardMask, starting_square: Square) -> Vec<Move> {
        board_mask
            .as_squares()
            .iter()
            .filter_map(|to| match self.board.get_piece_on_square(*to) {
                Some((piece, col)) => {
                    if *col == self.turn {
                        None
                    } else {
                        Some(Move::new(
                            self.board.get_piece_on_square(starting_square).unwrap().0,
                            starting_square,
                            *to,
                            Some(*piece),
                        ))
                    }
                }
                None => Some(Move::new(
                    self.board.get_piece_on_square(starting_square).unwrap().0,
                    starting_square,
                    *to,
                    None,
                )),
            })
            .collect()
    }
}
