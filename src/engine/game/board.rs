use super::piece::Piece;
use super::square::*;
use colored::Colorize;
use core::fmt;

pub trait Board: Default {
    // leaves room for board type specific optimization
    type Iter<'a>: Iterator<Item = (Square, Option<&'a Piece>)>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_>;
    /// returns the piece or none if the square is unoccupied
    fn get_piece_on_square(&self, square: Square) -> Option<Piece>;
    /// Sets square to hold piece
    fn put_piece_option(&mut self, square: Square, piece: Option<Piece>);
    /// Moves whatever is at *from* to *to*.
    /// Note that this method doesn't care, whether the move is valid or not.
    fn make_move(&mut self, from: Square, to: Square);

    /// 0..16 are white pieces
    /// 16..32 are black pieces
    /// each section starts with the king, the other pieces are unordered
    fn get_all_pieces(&self) -> [Option<(Square, &Piece)>; 32] {
        let mut pieces = [None; 32];
        let mut i = 1;
        let mut j = 17;
        self.iter().for_each(|(s, opt)| {
            if let Some(p) = opt {
                if p.is_king() && p.is_black() {
                    pieces[16] = Some((s, p));
                } else if p.is_king() && p.is_white() {
                    pieces[0] = Some((s, p));
                } else if p.is_white() {
                    pieces[i] = Some((s, p));
                    i += 1;
                } else {
                    pieces[j] = Some((s, p));
                    j += 1;
                }
            }
        });
        pieces
    }

    /// get all active white pieces
    /// 0 - is always King, all following pieces are in non specific order
    fn get_white_pieces(&self) -> [Option<(Square, &Piece)>; 16] {
        self.get_all_pieces()[0..16].try_into().unwrap()
    }
    /// get all active black pieces
    /// 0 - is always King, all following pieces are in non specific order
    fn get_black_pieces(&self) -> [Option<(Square, &Piece)>; 16] {
        self.get_all_pieces()[16..].try_into().unwrap()
    }

    fn put_piece(&mut self, square: Square, piece: Piece) {
        self.put_piece_option(square, Some(piece));
    }

    fn clear_square(&mut self, square: Square) {
        self.put_piece_option(square, None);
    }

    fn init() -> Self {
        let mut board = Self::default();
        board.put_piece(E1, Piece::WhiteKing);
        board.put_piece(D1, Piece::WhiteQueen);
        board.put_piece(C1, Piece::WhiteBishop);
        board.put_piece(F1, Piece::WhiteBishop);
        board.put_piece(B1, Piece::WhiteKnight);
        board.put_piece(G1, Piece::WhiteKnight);
        board.put_piece(H1, Piece::WhiteRook);
        board.put_piece(A1, Piece::WhiteRook);

        board.put_piece(E8, Piece::BlackKing);
        board.put_piece(D8, Piece::BlackQueen);
        board.put_piece(C8, Piece::BlackBishop);
        board.put_piece(F8, Piece::BlackBishop);
        board.put_piece(B8, Piece::BlackKnight);
        board.put_piece(G8, Piece::BlackKnight);
        board.put_piece(H8, Piece::BlackRook);
        board.put_piece(A8, Piece::BlackRook);

        board.put_piece(A7, Piece::BlackPawn);
        board.put_piece(B7, Piece::BlackPawn);
        board.put_piece(C7, Piece::BlackPawn);
        board.put_piece(D7, Piece::BlackPawn);
        board.put_piece(E7, Piece::BlackPawn);
        board.put_piece(F7, Piece::BlackPawn);
        board.put_piece(G7, Piece::BlackPawn);
        board.put_piece(H7, Piece::BlackPawn);

        board.put_piece(A2, Piece::WhitePawn);
        board.put_piece(B2, Piece::WhitePawn);
        board.put_piece(C2, Piece::WhitePawn);
        board.put_piece(D2, Piece::WhitePawn);
        board.put_piece(E2, Piece::WhitePawn);
        board.put_piece(F2, Piece::WhitePawn);
        board.put_piece(G2, Piece::WhitePawn);
        board.put_piece(H2, Piece::WhitePawn);

        board
    }

    fn format_print_board(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        for rank in Rank::One {
            for file in File::A {
                let s = Square::new(rank, file);
                write!(
                    f,
                    "{}",
                    if (rank as u8 / 8 + file as u8) % 2 == 0 {
                        match self.get_piece_on_square(s) {
                            None => "   ".normal(),
                            Some(p) => {
                                if p.is_black() {
                                    format!(" {} ", p.short_name()).black()
                                } else {
                                    format!(" {} ", p.short_name()).bright_white()
                                }
                            }
                        }
                        .on_bright_black()
                    } else {
                        match self.get_piece_on_square(s) {
                            None => "   ".normal(),
                            Some(p) => {
                                if p.is_black() {
                                    format!(" {} ", p.short_name()).black()
                                } else {
                                    format!(" {} ", p.short_name()).bright_white()
                                }
                            }
                        }
                        .on_white()
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
