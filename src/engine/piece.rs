use crate::engine::board::Board;
use crate::engine::square::{Square, SquareFromError};
use std::marker::PhantomData;

pub struct Pawn;
pub struct Knight;
pub struct Queen;
pub struct Rook;
pub struct Bishop;
pub struct King;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl Piece<Pawn> {
    pub fn new(color: Color) -> Self {
        Self {
            value: 1,
            color,
            piece_type: PhantomData,
        }
    }
}
impl Piece<Knight> {
    pub fn new(color: Color) -> Self {
        Self {
            value: 3,
            color,
            piece_type: PhantomData,
        }
    }
}

impl Piece<Bishop> {
    pub fn new(color: Color) -> Self {
        Self {
            value: 3,
            color,
            piece_type: PhantomData,
        }
    }
}

impl Piece<Rook> {
    pub fn new(color: Color) -> Self {
        Self {
            value: 5,
            color,
            piece_type: PhantomData,
        }
    }
}
impl Piece<Queen> {
    pub fn new(color: Color) -> Self {
        Self {
            value: 5,
            color,
            piece_type: PhantomData,
        }
    }
}

impl<T> Piece<T> {
    /// a helper function that only pushes to targeted, if there is no piece blocking the line of
    /// sight and, in case there is a piece on the field, only if this piece is of opposite color
    fn push_square_checked(&self, board: &Board, square_result: Result<Square, SquareFromError>, blocked: &mut bool, targeted: &mut Vec<Square>) {
        if !*blocked {
            match board.get_piece_on_square(square_result) {
                Ok(Some(piece)) => {
                    if piece.color != self.color {
                        targeted.push(square_result.unwrap());
                    }
                    *blocked = true;
                }
                Ok(None) => targeted.push(square_result.unwrap()),
                Err(_) => *blocked = true,
            }
        }
    }
}
pub trait Movable {
    fn targeted_squares(&self, position: &Square, board: &Board) -> Vec<Square>;
}

impl Movable for Piece<Pawn> {
    fn targeted_squares(&self, position: &Square, board: &Board) -> Vec<Square> {
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
    fn targeted_squares(&self, position: &Square, _: &Board) -> Vec<Square> {
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

impl Movable for Piece<Rook> {
    fn targeted_squares(&self, position: &Square, board: &Board) -> Vec<Square> {
        let mut targeted = Vec::new();
        let mut blocked_hor_neg = false;
        let mut blocked_hor_pos = false;
        let mut blocked_ver_neg = false;
        let mut blocked_ver_pos = false;

        for i in position
            .get_vertical()
            .max(position.get_horizontal())
            .max(position.get_vertical().abs_diff(7))
            .max(position.get_horizontal().abs_diff(7))..0
        {
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal() - i, position.get_vertical()),
                &mut blocked_hor_neg,
                &mut targeted,
            );
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal() + i, position.get_vertical()),
                &mut blocked_hor_pos,
                &mut targeted,
            );
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal(), position.get_vertical() - i),
                &mut blocked_ver_neg,
                &mut targeted,
            );
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal(), position.get_vertical() + i),
                &mut blocked_ver_pos,
                &mut targeted,
            );
        }
        targeted
    }
}
impl Movable for Piece<Bishop> {
    fn targeted_squares(&self, position: &Square, board: &Board) -> Vec<Square> {
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
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal() - i, position.get_vertical() - i),
                &mut blocked_neg_neg,
                &mut targeted,
            );
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal() + i, position.get_vertical() - i),
                &mut blocked_pos_neg,
                &mut targeted,
            );
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal() - i, position.get_vertical() + i),
                &mut blocked_neg_pos,
                &mut targeted,
            );
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal() + i, position.get_vertical() + i),
                &mut blocked_pos_pos,
                &mut targeted,
            );
        }
        targeted
    }
}

impl Movable for Piece<Queen> {
    fn targeted_squares(&self, position: &Square, board: &Board) -> Vec<Square> {
        let r = Piece::<Rook>::new(self.color);
        let b = Piece::<Bishop>::new(self.color);
        let mut targeted = r.targeted_squares(position, board);
        targeted.append(&mut b.targeted_squares(position, board));
        targeted
    }
}

impl Movable for Piece<King> {
    fn targeted_squares(&self, position: &Square, board: &Board) -> Vec<Square> {
        let mut targeted = Vec::new();
        if position.get_horizontal() >= 1 {
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal() - 1, position.get_vertical()),
                &mut false,
                &mut targeted,
            );
            if position.get_vertical() >= 1 {
                self.push_square_checked(
                    board,
                    Square::new(position.get_horizontal() - 1, position.get_vertical() - 1),
                    &mut false,
                    &mut targeted,
                );
            }
            if position.get_vertical() <= 6 {
                self.push_square_checked(
                    board,
                    Square::new(position.get_horizontal() - 1, position.get_vertical() + 1),
                    &mut false,
                    &mut targeted,
                );
            }
        }
        if position.get_horizontal() <= 6 {
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal() + 1, position.get_vertical()),
                &mut false,
                &mut targeted,
            );
            if position.get_vertical() >= 1 {
                self.push_square_checked(
                    board,
                    Square::new(position.get_horizontal() + 1, position.get_vertical() - 1),
                    &mut false,
                    &mut targeted,
                );
            }
            if position.get_vertical() <= 6 {
                self.push_square_checked(
                    board,
                    Square::new(position.get_horizontal() + 1, position.get_vertical() + 1),
                    &mut false,
                    &mut targeted,
                );
            }
        }
        if position.get_vertical() >= 1 {
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal(), position.get_vertical() - 1),
                &mut false,
                &mut targeted,
            );
        }
        if position.get_vertical() <= 6 {
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal(), position.get_vertical() + 1),
                &mut false,
                &mut targeted,
            );
        }
        targeted
    }
}
