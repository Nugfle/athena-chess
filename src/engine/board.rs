use crate::engine::piece::Piece;
use crate::engine::square::Square;

pub struct Board {
    board: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn square_is_occupied<T>(&self, square: T) -> Result<bool, T::Error>
    where
        T: TryInto<Square>,
    {
        Ok(self.get_piece_on_square(square).is_ok())
    }

    pub fn get_piece_on_square<T>(&self, square: T) -> Result<Option<&Piece>, T::Error>
    where
        T: TryInto<Square>,
    {
        let square = square.try_into()?;
        Ok(self.board[square.get_vertical() as usize][square.get_horizontal() as usize].as_ref())
    }
}
