use crate::engine::board::Board;
use crate::engine::square::Square;
use std::marker::PhantomData;

pub struct Pawn;
pub struct Knight;
pub struct Queen;
pub struct Rook;
pub struct Bishop;
pub struct King;

pub struct Piece<PieceType = Pawn> {
    value: u8,
    position: Square,
    piece_type: PhantomData<PieceType>,
}

impl Piece<Rook> {
    pub fn targeted_squares(&self, board: &Board) -> Vec<Square> {
        let mut targeted = Vec::new();
        for h in self.position.get_horizontal()..0 {
            let square = Square::new(h, self.position.get_vertical()).unwrap();
            if board.square_is_occupied(square) {
                break;
            }
            targeted.push(square);
        }
        for h in self.position.get_horizontal()..8 {
            let square = Square::new(h, self.position.get_vertical()).unwrap();
            if board.square_is_occupied(square) {
                break;
            }
            targeted.push(square);
        }
        for v in self.position.get_vertical()..0 {
            let square = Square::new(self.position.get_horizontal(), v).unwrap();
            if board.square_is_occupied(square) {
                break;
            }
            targeted.push(square);
        }
        for v in self.position.get_vertical()..8 {
            let square = Square::new(self.position.get_horizontal(), v).unwrap();
            if board.square_is_occupied(square) {
                break;
            }
            targeted.push(square);
        }
        targeted
    }
}
