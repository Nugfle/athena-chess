use crate::game::BoardMask;
use crate::game::board::Occupancy;
use crate::game::board::square::Square;

/// the knight is no sliding piece, so we don't need to consider occupancy patterns for the knights
/// movement. Therefore the logic is fairly straight forward
pub fn create_knight_attack_pattern(square: Square) -> BoardMask {
    let mut pattern = BoardMask(0);
    // -2 -1
    if square > Square::new(17).unwrap() {
        pattern.add_square(Square::new(square.as_u8() - 17).unwrap());
    }
    // -2 + 1
    if square > Square::new(17).unwrap() {
        pattern.add_square(Square::new(square.as_u8() - 15).unwrap());
    }
    // -1, -2
    if square > Square::new(17).unwrap() {
        pattern.add_square(Square::new(square.as_u8() - 10).unwrap());
    }
    // -1, +2
    if square > Square::new(17).unwrap() {
        pattern.add_square(Square::new(square.as_u8() - 6).unwrap());
    }

    // +1 -2
    if square < Square::new(58).unwrap() {
        pattern.add_square(Square::new(square.as_u8() + 6).unwrap());
    }
    // +1 +2
    if square < Square::new(54).unwrap() {
        pattern.add_square(Square::new(square.as_u8() + 10).unwrap());
    }
    // +2 -1
    if square < Square::new(49).unwrap() {
        pattern.add_square(Square::new(square.as_u8() + 15).unwrap());
    }
    // +2 +1
    if square < Square::new(47).unwrap() {
        pattern.add_square(Square::new(square.as_u8() + 17).unwrap());
    }

    pattern
}

/// returns a mask used for indexing rook attack patterns. The mask contains all movable squares
/// from starting square whith a rook, except the border squares.
pub fn create_rook_mask(square: Square) -> BoardMask {
    let mut mask = BoardMask(0);
    let mut i = square.as_u8();
    // cover the rows
    while i % 8 < 7 {
        mask.add_square(Square::new(i).unwrap());
        i += 1;
    }
    i = square.as_u8();
    while i % 8 > 0 {
        mask.add_square(Square::new(i).unwrap());
        i -= 1;
    }
    i = square.as_u8();
    // cover the columns
    while i / 8 < 7 {
        mask.add_square(Square::new(i).unwrap());
        i += 8;
    }
    i = square.as_u8();
    while i / 8 > 0 {
        mask.add_square(Square::new(i).unwrap());
        i -= 8;
    }
    // our current logic adds the starting square, so we just remove it
    mask.with_square_removed(square)
}

/// returns the bitboard pattern for all possible rook moves with given occupancy starting at square
pub fn create_rook_attack_pattern(square: Square, occupancy: Occupancy) -> BoardMask {
    let mut mask = BoardMask(0);

    // can't panic because square is in range 0-63
    let s: i8 = square.as_u8().try_into().unwrap();
    let mut i = s;

    while i / 8 == s / 8 && i < 64 {
        let sq = Square::new(i as u8).unwrap();
        mask.add_square(sq);
        if occupancy.is_occupied(sq) {
            break;
        }
        i += 1;
    }
    i = s;
    while i / 8 == s / 8 && i >= 0 {
        let sq = Square::new(i as u8).unwrap();
        mask.add_square(sq);
        if occupancy.is_occupied(sq) {
            break;
        }
        i -= 1;
    }
    i = s;
    // cover the columns
    while i < 64 {
        let sq = Square::new(i as u8).unwrap();
        mask.add_square(sq);
        if occupancy.is_occupied(sq) {
            break;
        }
        i += 8;
    }
    i = s;
    while i >= 0 {
        let sq = Square::new(i as u8).unwrap();
        mask.add_square(sq);
        if occupancy.is_occupied(sq) {
            break;
        }
        i -= 8;
    }
    // our current logic adds the starting square, so we just remove it
    mask.with_square_removed(square)
}

/// returns a mask used for indexing bishop attack patterns. The mask contains all movable squares
/// from starting square whith a bishop, except the border squares.
pub fn create_bishop_mask(square: Square) -> BoardMask {
    let mut mask: BoardMask = BoardMask(0);

    let mut i = square.as_u8();

    // goes into ++ direction until it hits the edge of the board
    while i % 8 < 7 && i / 8 < 7 {
        // can't panic because square is valid by checks above
        mask.add_square(Square::new(i).unwrap());
        i += 9;
    }
    i = square.as_u8();
    // goes into -- direction until it hits the edge of the board
    while i % 8 > 0 && i / 8 > 0 {
        mask.add_square(Square::new(i).unwrap());
        i -= 9;
    }
    i = square.as_u8();
    // goes into -+ direction until it hits the edge of the board
    while i % 8 < 7 && i / 8 > 0 {
        mask.add_square(Square::new(i).unwrap());
        i -= 7;
    }
    i = square.as_u8();
    // goes into +- direction until it hits the edge of the board
    while i % 8 > 0 && i / 8 < 7 {
        mask.add_square(Square::new(i).unwrap());
        i += 7;
    }
    // our current logic adds the starting square, so we just remove it
    mask.with_square_removed(square)
}
/// returns the bitboard pattern for all possible rook moves with given occupancy starting at square
pub fn create_bishop_attack_pattern(square: Square, occupancy: Occupancy) -> BoardMask {
    let mut mask = BoardMask(0);
    // can't panic because square is always in range 0-63
    let s: i8 = square.as_u8().try_into().unwrap();
    let mut i = s;

    // goes into ++ direction until it hits the edge of the board
    while i < 64 {
        mask.add_square(Square::new(i as u8).unwrap());
        if occupancy.0 & (1 << i) != 0 || i % 8 == 7 {
            break;
        }
        i += 9;
    }
    i = s;
    // goes into -- direction until it hits the edge of the board
    while i >= 0 {
        mask.add_square(Square::new(i as u8).unwrap());
        if occupancy.0 & (1 << i) != 0 || i % 8 == 0 {
            break;
        }
        i -= 9;
    }
    i = s;
    // goes into -+ direction until it hits the edge of the board
    while i >= 0 {
        mask.add_square(Square::new(i as u8).unwrap());
        if occupancy.0 & (1 << i) != 0 || i % 8 == 7 {
            break;
        }
        i -= 7;
    }
    i = s;
    // goes into +- direction until it hits the edge of the board
    while i < 64 {
        mask.add_square(Square::new(i as u8).unwrap());
        if occupancy.0 & (1 << i) != 0 || i % 8 == 0 {
            break;
        }
        i += 7;
    }
    // our current logic adds the starting square, so we just remove it
    mask.with_square_removed(square)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::board::square::*;

    fn squares_from_mask(mask: BoardMask) -> Vec<String> {
        let mut squares = Vec::new();
        for i in 0..64 {
            if mask.contains(Square::new(i).unwrap()) {
                squares.push(Square::new(i).unwrap().to_string());
            }
        }
        squares
    }

    fn check_bit_board_pattern(expected: BoardMask, computed: BoardMask) {
        assert_eq!(
            computed,
            expected,
            "assert failed:\nexpected: {:>64b}\ngot:      {:>64b}\nerror:    {:>64b}\nsquares:  {:?}",
            expected.0,
            computed.0,
            (computed ^ expected).0,
            squares_from_mask(computed ^ expected),
        )
    }

    #[test]
    fn test_create_bishop_mask() {
        // put the bishop on d3;
        let s = D4;
        let m = create_bishop_mask(s);
        let expected = BoardMask(0)
            .with_square(E3)
            .with_square(F2)
            .with_square(C3)
            .with_square(B2)
            .with_square(C5)
            .with_square(B6)
            .with_square(E5)
            .with_square(F6)
            .with_square(G7);
        check_bit_board_pattern(expected, m);
    }

    #[test]
    fn test_create_rook_mask() {
        // put the bishop on d3;
        let s = D4;
        let m = create_rook_mask(s);

        let expected = BoardMask(0)
            .with_square(D5)
            .with_square(D6)
            .with_square(D7)
            .with_square(D3)
            .with_square(D2)
            .with_square(C4)
            .with_square(B4)
            .with_square(E4)
            .with_square(F4)
            .with_square(G4);
        check_bit_board_pattern(expected, m);
    }

    #[test]
    fn test_create_bishop_attack_pattern_no_occupants() {
        // put the bishop on d3;
        let s = D4;
        let m = create_bishop_attack_pattern(s, Occupancy(0));

        let expected = BoardMask(0)
            .with_square(E3)
            .with_square(F2)
            .with_square(G1)
            .with_square(C3)
            .with_square(B2)
            .with_square(A1)
            .with_square(C5)
            .with_square(B6)
            .with_square(A7)
            .with_square(E5)
            .with_square(F6)
            .with_square(G7)
            .with_square(H8);

        check_bit_board_pattern(expected, m);
    }
    #[test]
    fn test_create_bishop_attack_pattern_with_occupants() {
        // put the bishop on d3;
        let s = D4;
        let occupancy = Occupancy(0).with_square(F2).with_square(A1).with_square(C5).with_square(F6);
        let m = create_bishop_attack_pattern(s, occupancy);

        let expected = BoardMask(0)
            .with_square(E3)
            .with_square(F2)
            .with_square(C3)
            .with_square(B2)
            .with_square(A1)
            .with_square(C5)
            .with_square(E5)
            .with_square(F6);

        check_bit_board_pattern(expected, m);
    }

    #[test]
    fn test_create_rook_attack_pattern_no_occupants() {
        // put the bishop on d3;
        let s = D4;
        let m = create_rook_attack_pattern(s, Occupancy(0));
        let expected = BoardMask(0)
            .with_square(D5)
            .with_square(D6)
            .with_square(D7)
            .with_square(D8)
            .with_square(D3)
            .with_square(D2)
            .with_square(D1)
            .with_square(C4)
            .with_square(B4)
            .with_square(A4)
            .with_square(E4)
            .with_square(F4)
            .with_square(G4)
            .with_square(H4);
        check_bit_board_pattern(expected, m);
    }
    #[test]
    fn test_create_rook_attack_pattern_with_occupants() {
        // put the bishop on d3;
        let s = D4;
        let occupancy = Occupancy(0)
            .with_square(D7)
            .with_square(D3)
            .with_square(A4)
            .with_square(F4)
            .with_square(H4)
            .with_square(H8);
        let m = create_rook_attack_pattern(s, occupancy);
        let expected = BoardMask(0)
            .with_square(D5)
            .with_square(D6)
            .with_square(D7)
            .with_square(D3)
            .with_square(C4)
            .with_square(B4)
            .with_square(A4)
            .with_square(E4)
            .with_square(F4);
        check_bit_board_pattern(expected, m);
    }
}
