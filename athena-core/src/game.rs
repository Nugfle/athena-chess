use attack_tables::AttackTables;
use board::BitBoard;
use log::{error, info};
pub use mask::BoardMask;
use std::sync::LazyLock;

pub use board::piece::{Color, Piece};
pub use board::square::*;
pub use chess_move::Move;

mod attack_tables;
mod board;
mod chess_move;
mod error;
mod mask;

//#[cfg(test)]
//mod tests;

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

    pub fn execute_move(&mut self, mv: Move) {
        match mv {
            Move::Normal { piece: _, from, to } => {
                let (mut piece, color) = self.board.remove_piece_from_square(from).expect("from square must contain a piece");
                piece.make_moved();
                self.board.place_piece_on_square(piece, color, to);
            }
            Move::Capture { piece: _, from, to, captured: _ } => {
                let (mut piece, color) = self.board.remove_piece_from_square(from).expect("from square must contain a piece");
                piece.make_moved();
                self.board.place_piece_on_square(piece, color, to);
            }
            Move::EnPassant { from, to } => {
                let (piece, color) = self.board.remove_piece_from_square(from).expect("from square must contain a pawn");
                assert!(self.board.get_piece_on_square(to).is_none(), "to square must be empty");
                self.board.place_piece_on_square(piece, color, to);
                
                // Remove the captured pawn from the square beside the destination
                let captured_pawn_square = Square::from_rank_file(from.get_rank(), to.get_file());
                self.board.remove_piece_from_square(captured_pawn_square).expect("captured pawn must exist");
            }
            Move::Promotion { from, to, promoted_to } => {
                let (_, color) = self.board.remove_piece_from_square(from).expect("from square must contain a pawn");
                let mut promoted_piece = promoted_to;
                promoted_piece.make_moved();
                self.board.place_piece_on_square(promoted_piece, color, to);
            }
            Move::PromotionCapture { from, to, captured: _, promoted_to } => {
                let (_, color) = self.board.remove_piece_from_square(from).expect("from square must contain a pawn");
                let mut promoted_piece = promoted_to;
                promoted_piece.make_moved();
                self.board.place_piece_on_square(promoted_piece, color, to);
            }
            Move::CastleKingside { king_from, king_to, rook_from, rook_to } => {
                // Move the king
                let (mut king, color) = self.board.remove_piece_from_square(king_from).expect("king must be on king_from square");
                king.make_moved();
                self.board.place_piece_on_square(king, color, king_to);
                
                // Move the rook
                let (mut rook, _) = self.board.remove_piece_from_square(rook_from).expect("rook must be on rook_from square");
                rook.make_moved();
                self.board.place_piece_on_square(rook, color, rook_to);
            }
            Move::CastleQueenside { king_from, king_to, rook_from, rook_to } => {
                // Move the king
                let (mut king, color) = self.board.remove_piece_from_square(king_from).expect("king must be on king_from square");
                king.make_moved();
                self.board.place_piece_on_square(king, color, king_to);
                
                // Move the rook
                let (mut rook, _) = self.board.remove_piece_from_square(rook_from).expect("rook must be on rook_from square");
                rook.make_moved();
                self.board.place_piece_on_square(rook, color, rook_to);
            }
        }

        self.moves.push(mv);
        self.turn = !self.turn;
    }

    pub(crate) fn get_available_moves(&self) -> Vec<Move> {
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
            .map(|(sq, pc)| match pc {
                Piece::Rook { .. } => {
                    self.moves_from_mask_and_starting_square(&ATTACK_TABLES.get_attack_pattern_rook(*sq, self.board.occupancy), *sq)
                }
                Piece::Pawn => self.get_pawn_moves(*sq),
                Piece::King { .. } => self.get_king_moves(*sq),
                Piece::Knight => self.moves_from_mask_and_starting_square(&ATTACK_TABLES.get_attack_pattern_knight(*sq), *sq),
                Piece::Queen => self.moves_from_mask_and_starting_square(&ATTACK_TABLES.get_attack_pattern_queen(*sq, self.board.occupancy), *sq),
                Piece::Bishop => self.moves_from_mask_and_starting_square(&ATTACK_TABLES.get_attack_pattern_bishop(*sq, self.board.occupancy), *sq),
            })
            .flatten()
            .collect()
    }

    pub(crate) fn previous_move(&self) -> Option<&Move> {
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

    fn get_king_moves(&self, from: Square) -> Vec<Move> {
        let (piece, color) = self.board.get_piece_on_square(from).expect("from must contain a piece");
        assert!(piece.is_king(), "the piece must be a king");
        let mut available_moves = Vec::new();
        todo!("implement king moves");
        available_moves
    }

    fn get_pawn_moves(&self, from: Square) -> Vec<Move> {
        let (piece, color) = self.board.get_piece_on_square(from).expect("from must contain a piece");
        assert!(piece.is_pawn(), "the piece must be a pawn");
        let heading = if color.is_white() { 1 } else { -1 };
        let mut available_moves = Vec::new();

        if let Ok(forward) = from.move_on_rank(heading) {
            if self.board.get_piece_on_square(forward).is_none() {
                if forward.get_rank().is_on_edge() {
                    available_moves.copy_from_slice(&Move::promotions(from, forward, None));
                } else {
                    available_moves.push(Move::new(Piece::Pawn, from, forward, None));
                }
                if (from.get_rank() == Rank::Two && *color == Color::White) || (from.get_rank() == Rank::Seven && *color == Color::Black) {
                    if self
                        .board
                        .get_piece_on_square(forward.move_on_rank(heading).expect("we checked"))
                        .is_none()
                    {
                        available_moves.push(Move::new(
                            Piece::Pawn,
                            from,
                            forward.move_on_rank(heading).expect("we checked"),
                            None,
                        ));
                    }
                }
            }
            if let Ok(forward_pos) = forward.move_on_file(1) {
                match self.board.get_piece_on_square(forward_pos) {
                    None => {
                        if forward.get_rank().is_on_edge() {
                            available_moves.copy_from_slice(&Move::promotions(from, forward_pos, None));
                        } else {
                            available_moves.push(Move::new(Piece::Pawn, from, forward_pos, None));
                        }
                    }
                    Some((pc, col)) if col != color => {
                        if forward.get_rank().is_on_edge() {
                            available_moves.copy_from_slice(&Move::promotions(from, forward_pos, Some(*pc)));
                        } else {
                            available_moves.push(Move::new(Piece::Pawn, from, forward_pos, Some(*pc)));
                        }
                    }
                    _ => {}
                }
            }
            if let Ok(forward_neg) = forward.move_on_file(-1) {
                match self.board.get_piece_on_square(forward_neg) {
                    None => {
                        if forward.get_rank().is_on_edge() {
                            available_moves.copy_from_slice(&Move::promotions(from, forward_neg, None));
                        } else {
                            available_moves.push(Move::new(Piece::Pawn, from, forward_neg, None));
                        }
                    }
                    Some((pc, col)) if col != color => {
                        if forward.get_rank().is_on_edge() {
                            available_moves.copy_from_slice(&Move::promotions(from, forward_neg, Some(*pc)));
                        } else {
                            available_moves.push(Move::new(Piece::Pawn, from, forward_neg, Some(*pc)));
                        }
                    }
                    _ => {}
                }
            }

            // en pasente
            if self.previous_move().is_some_and(|m| {
                m.get_piece().is_pawn()
                    && m.get_from().get_delta_rank(m.get_to()).abs() == 2
                    && m.get_to().get_delta_rank(from) == 0
                    && m.get_to().get_delta_file(from).abs() == 1
            }) {
                match self.previous_move().expect("checked").get_to().get_delta_file(from) {
                    1 => available_moves.push(Move::en_pesante(from, forward.move_on_file(1).expect("checked"))),
                    -1 => available_moves.push(Move::en_pesante(from, forward.move_on_file(-1).expect("checked"))),
                    _ => {}
                }
            }
        } else {
            error!("error Pawn on the backrank: {}", from);
        }
        available_moves
    }
}
