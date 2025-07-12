use std::usize;

use crate::engine::board::Board;
use crate::engine::square::{Square, SquareFromError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

#[derive(Clone, Copy, Debug)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    pub fn get_color(&self) -> Color {
        match self {
            Piece::Pawn(color) => *color,
            Piece::Knight(color) => *color,
            Piece::Bishop(color) => *color,
            Piece::Rook(color) => *color,
            Piece::Queen(color) => *color,
            Piece::King(color) => *color,
        }
    }
    pub fn get_movable_squares(&self, starting_square: &Square, board: &Board) -> Vec<Square> {
        match self {
            Piece::Pawn(Color::Black) => self.black_pawn_movable_squares(starting_square, board),
            Piece::Pawn(Color::White) => self.white_pawn_movable_squares(starting_square, board),
            Piece::Knight(_) => self.knight_movable_squares(starting_square, board),
            Piece::Bishop(_) => self.bishop_movable_squares(starting_square, board),
            Piece::Rook(_) => self.rook_movable_squares(starting_square, board),
            Piece::Queen(_) => self.queen_movable_squares(starting_square, board),
            Piece::King(_) => self.king_movable_squares(starting_square, board),
        }
    }

    fn push_square_checked(&self, board: &Board, square_result: Result<Square, SquareFromError>, blocked: &mut bool, movable: &mut Vec<Square>) {
        if !*blocked {
            match board.get_piece_on_square(square_result) {
                Ok(Some(piece)) => {
                    if piece.get_color() != self.get_color() {
                        movable.push(square_result.unwrap());
                    }
                    *blocked = true;
                }
                Ok(None) => movable.push(square_result.unwrap()),
                Err(_) => *blocked = true,
            }
        }
    }
    fn black_pawn_movable_squares(&self, position: &Square, board: &Board) -> Vec<Square> {
        let mut movable = Vec::new();
        if let Ok(false) = board.square_is_occupied((position.get_horizontal(), position.get_vertical() - 1)) {
            movable.push(Square::new(position.get_horizontal(), position.get_vertical() - 1).unwrap());
        }
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() - 1, position.get_vertical() - 1),
            &mut false,
            &mut movable,
        );
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() + 1, position.get_vertical() - 1),
            &mut false,
            &mut movable,
        );
        movable
    }
    fn white_pawn_movable_squares(&self, position: &Square, board: &Board) -> Vec<Square> {
        let mut movable = Vec::new();
        if let Ok(false) = board.square_is_occupied((position.get_horizontal(), position.get_vertical() + 1)) {
            movable.push(Square::new(position.get_horizontal(), position.get_vertical() + 1).unwrap());
        }
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() - 1, position.get_vertical() + 1),
            &mut false,
            &mut movable,
        );
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() + 1, position.get_vertical() + 1),
            &mut false,
            &mut movable,
        );
        movable
    }
    fn knight_movable_squares(&self, position: &Square, board: &Board) -> Vec<Square> {
        let mut movable = Vec::new();
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() + 2, position.get_vertical() + 1),
            &mut false,
            &mut movable,
        );
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() + 2, position.get_vertical() - 1),
            &mut false,
            &mut movable,
        );
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() - 2, position.get_vertical() + 1),
            &mut false,
            &mut movable,
        );
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() - 2, position.get_vertical() - 1),
            &mut false,
            &mut movable,
        );
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() + 1, position.get_vertical() + 2),
            &mut false,
            &mut movable,
        );
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() + 1, position.get_vertical() - 2),
            &mut false,
            &mut movable,
        );
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() - 1, position.get_vertical() + 2),
            &mut false,
            &mut movable,
        );
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() - 1, position.get_vertical() - 2),
            &mut false,
            &mut movable,
        );
        movable
    }
    fn rook_movable_squares(&self, position: &Square, board: &Board) -> Vec<Square> {
        let mut movable = Vec::new();
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
                &mut movable,
            );
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal() + i, position.get_vertical()),
                &mut blocked_hor_pos,
                &mut movable,
            );
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal(), position.get_vertical() - i),
                &mut blocked_ver_neg,
                &mut movable,
            );
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal(), position.get_vertical() + i),
                &mut blocked_ver_pos,
                &mut movable,
            );
        }
        movable
    }
    fn bishop_movable_squares(&self, position: &Square, board: &Board) -> Vec<Square> {
        let mut movable = Vec::new();
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
                &mut movable,
            );
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal() + i, position.get_vertical() - i),
                &mut blocked_pos_neg,
                &mut movable,
            );
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal() - i, position.get_vertical() + i),
                &mut blocked_neg_pos,
                &mut movable,
            );
            self.push_square_checked(
                board,
                Square::new(position.get_horizontal() + i, position.get_vertical() + i),
                &mut blocked_pos_pos,
                &mut movable,
            );
        }
        movable
    }
    fn queen_movable_squares(&self, position: &Square, board: &Board) -> Vec<Square> {
        let mut movable = self.rook_movable_squares(position, board);
        movable.append(&mut self.bishop_movable_squares(position, board));
        movable
    }
    fn king_movable_squares(&self, position: &Square, board: &Board) -> Vec<Square> {
        let mut movable = Vec::new();
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() - 1, position.get_vertical()),
            &mut false,
            &mut movable,
        );
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() + 1, position.get_vertical()),
            &mut false,
            &mut movable,
        );
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal(), position.get_vertical() - 1),
            &mut false,
            &mut movable,
        );
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal(), position.get_vertical() + 1),
            &mut false,
            &mut movable,
        );
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() - 1, position.get_vertical() - 1),
            &mut false,
            &mut movable,
        );
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() - 1, position.get_vertical() + 1),
            &mut false,
            &mut movable,
        );
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() + 1, position.get_vertical() - 1),
            &mut false,
            &mut movable,
        );
        self.push_square_checked(
            board,
            Square::new(position.get_horizontal() + 1, position.get_vertical() + 1),
            &mut false,
            &mut movable,
        );
        movable
    }
}
