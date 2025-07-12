use crate::engine::board::Board;
use crate::engine::square::Square;
use std::marker::PhantomData;

pub struct Pawn;
pub struct Knight;
pub struct Queen;
pub struct Rook;
pub struct Bishop;
pub struct King;

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Black,
    White,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece<PieceType = Pawn> {
    value: u8,
    color: Color,
    piece_type: PhantomData<PieceType>,
}

pub trait Movable {
    fn targeted_squares(&self, position: Square, board: &Board) -> Vec<Square>;
}

impl Movable for Piece<Pawn> {
    fn targeted_squares(&self, position: Square, board: &Board) -> Vec<Square> {
        let mut targeted = Vec::new();
        match self.color {
            Color::Black => {
                if let Ok(false) = board.square_is_occupied((position.get_horizontal(), position.get_vertical() - 1)) {
                    targeted.push(Square::new(position.get_horizontal(), position.get_vertical() - 1).unwrap());
                }
                if let Ok(true) = board.square_is_occupied((position.get_horizontal() - 1, position.get_vertical() - 1)) {
                    targeted.push(Square::new(position.get_horizontal() - 1, position.get_vertical() - 1).unwrap());
                }
                if let Ok(true) = board.square_is_occupied((position.get_horizontal() + 1, position.get_vertical() - 1)) {
                    targeted.push(Square::new(position.get_horizontal() + 1, position.get_vertical() - 1).unwrap());
                }
                if position.get_vertical() == 6 {
                    if let Ok(false) = board.square_is_occupied((position.get_horizontal(), position.get_vertical() - 2)) {
                        targeted.push(Square::new(position.get_horizontal(), position.get_vertical() - 2).unwrap());
                    }
                }
            }
            Color::White => {
                if let Ok(false) = board.square_is_occupied((position.get_horizontal(), position.get_vertical() + 1)) {
                    targeted.push(Square::new(position.get_horizontal(), position.get_vertical() + 1).unwrap());
                }
                if let Ok(true) = board.square_is_occupied((position.get_horizontal() - 1, position.get_vertical() + 1)) {
                    targeted.push(Square::new(position.get_horizontal() - 1, position.get_vertical() + 1).unwrap());
                }
                if let Ok(true) = board.square_is_occupied((position.get_horizontal() + 1, position.get_vertical() + 1)) {
                    targeted.push(Square::new(position.get_horizontal() + 1, position.get_vertical() + 1).unwrap());
                }
                if position.get_vertical() == 6 {
                    if let Ok(false) = board.square_is_occupied((position.get_horizontal(), position.get_vertical() + 2)) {
                        targeted.push(Square::new(position.get_horizontal(), position.get_vertical() + 2).unwrap());
                    }
                }
            }
        }
        targeted
    }
}

impl Movable for Piece<Knight> {
    fn targeted_squares(&self, position: Square, _: &Board) -> Vec<Square> {
        let mut targeted = Vec::new();
        if position.get_horizontal() <= 5 {
            if position.get_vertical() <= 6 {
                targeted.push(Square::new(position.get_horizontal() + 2, position.get_vertical() + 1).unwrap());
            }
            if position.get_vertical() >= 1 {
                targeted.push(Square::new(position.get_horizontal() + 2, position.get_vertical() - 1).unwrap());
            }
        }
        if position.get_horizontal() >= 2 {
            if position.get_vertical() <= 6 {
                targeted.push(Square::new(position.get_horizontal() - 2, position.get_vertical() + 1).unwrap());
            }
            if position.get_vertical() >= 1 {
                targeted.push(Square::new(position.get_horizontal() - 2, position.get_vertical() - 1).unwrap());
            }
        }
        if position.get_vertical() <= 5 {
            if position.get_horizontal() <= 6 {
                targeted.push(Square::new(position.get_horizontal() + 1, position.get_vertical() + 2).unwrap());
            }
            if position.get_horizontal() >= 1 {
                targeted.push(Square::new(position.get_horizontal() - 1, position.get_vertical() + 2).unwrap());
            }
        }
        if position.get_vertical() >= 2 {
            if position.get_horizontal() <= 6 {
                targeted.push(Square::new(position.get_horizontal() + 1, position.get_vertical() - 2).unwrap());
            }
            if position.get_horizontal() >= 1 {
                targeted.push(Square::new(position.get_horizontal() - 1, position.get_vertical() - 2).unwrap());
            }
        }
        targeted
    }
}

impl Movable for Piece<Bishop> {
    fn targeted_squares(&self, position: Square, board: &Board) -> Vec<Square> {
        let mut targeted = Vec::new();
        let mut blocked_neg_neg = false;
        let mut blocked_neg_pos = false;
        let mut blocked_pos_neg = false;
        let mut blocked_pos_pos = false;
        for i in position
            .get_vertical()
            .max(position.get_horizontal())
            .max(position.get_vertical().abs_diff(7))
            .max(position.get_horizontal().abs_diff(7))..0
        {
            if let Ok(occupied) = board.square_is_occupied((position.get_horizontal() - i, position.get_vertical() - i)) {
                if !blocked_neg_neg {
                    targeted.push(Square::new(position.get_horizontal() - i, position.get_vertical() - i).unwrap());
                }
                if occupied {
                    blocked_neg_neg = true;
                }
            }
            if let Ok(occupied) = board.square_is_occupied((position.get_horizontal() + i, position.get_vertical() - i)) {
                if !blocked_pos_neg {
                    targeted.push(Square::new(position.get_horizontal() + i, position.get_vertical() - i).unwrap());
                }
                if occupied {
                    blocked_pos_neg = true;
                }
            }
            if let Ok(occupied) = board.square_is_occupied((position.get_horizontal() - i, position.get_vertical() + i)) {
                if !blocked_neg_pos {
                    targeted.push(Square::new(position.get_horizontal() - i, position.get_vertical() + i).unwrap());
                }
                if occupied {
                    blocked_neg_pos = true;
                }
            }
            if let Ok(occupied) = board.square_is_occupied((position.get_horizontal() + i, position.get_vertical() + i)) {
                if !blocked_pos_pos {
                    targeted.push(Square::new(position.get_horizontal() + i, position.get_vertical() + i).unwrap());
                }
                if occupied {
                    blocked_pos_pos = true;
                }
            }
        }
        targeted
    }
}

impl Movable for Piece<Rook> {
    fn targeted_squares(&self, position: Square, board: &Board) -> Vec<Square> {
        let mut targeted = Vec::new();
        for h in position.get_horizontal()..0 {
            let square = Square::new(h, position.get_vertical()).unwrap();
            if let Ok(true) = board.square_is_occupied(square) {
                break;
            }
            targeted.push(square);
        }
        for h in position.get_horizontal()..8 {
            let square = Square::new(h, position.get_vertical()).unwrap();
            if let Ok(true) = board.square_is_occupied(square) {
                break;
            }
            targeted.push(square);
        }
        for v in position.get_vertical()..0 {
            let square = Square::new(position.get_horizontal(), v).unwrap();
            if let Ok(true) = board.square_is_occupied(square) {
                break;
            }
            targeted.push(square);
        }
        for v in position.get_vertical()..8 {
            let square = Square::new(position.get_horizontal(), v).unwrap();
            if let Ok(true) = board.square_is_occupied(square) {
                break;
            }
            targeted.push(square);
        }
        targeted
    }
}
