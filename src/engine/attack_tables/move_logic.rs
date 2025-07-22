use crate::engine::board::Occupancy;
use crate::engine::board::square::*;

/// the knight is no sliding piece, so we don't need to consider occupancy patterns for the knights
/// movement. Therefore the logic is fairly straight forward
pub fn create_knight_attack_pattern(square: Square) -> u64 {
    let mut pattern = 0;
    // -2 -1
    if square.0 > 17 {
        pattern |= 1 << (square.0 - 17);
    }
    // -2 + 1
    if square.0 > 15 {
        pattern |= 1 << (square.0 - 15);
    }
    // -1, -2
    if square.0 > 10 {
        pattern |= 1 << (square.0 - 10);
    }
    // -1, +2
    if square.0 > 6 {
        pattern |= 1 << (square.0 - 6);
    }

    // +1 -2
    if square.0 < 58 {
        pattern |= 1 << (square.0 + 6);
    }
    // +1 +2
    if square.0 < 54 {
        pattern |= 1 << (square.0 + 10);
    }
    // +2 -1
    if square.0 < 49 {
        pattern |= 1 << (square.0 + 15);
    }
    // +2 +1
    if square.0 < 47 {
        pattern |= 1 << (square.0 + 17);
    }

    pattern
}

/// returns a mask used for indexing rook attack patterns. The mask contains all movable squares
/// from starting square whith a rook, except the border squares.
pub fn create_rook_mask(square: Square) -> u64 {
    let mut mask: u64 = 0;
    let mut i = square.0;
    // cover the rows
    while i % 8 < 7 {
        mask |= 1 << i;
        i += 1;
    }
    i = square.0;
    while i % 8 > 0 {
        mask |= 1 << i;
        i -= 1;
    }
    i = square.0;
    // cover the columns
    while i / 8 < 7 {
        mask |= 1 << i;
        i += 8;
    }
    i = square.0;
    while i / 8 > 0 {
        mask |= 1 << i;
        i -= 8;
    }
    // our current logic adds the starting square, so we just remove it
    mask & !(1 << square.0)
}

/// returns the bitboard pattern for all possible rook moves with given occupancy starting at square
pub fn create_rook_attack_pattern(square: Square, occupancy: Occupancy) -> u64 {
    let mut mask: u64 = 0;

    let s = square.0 as i8;
    let mut i = s;

    while i / 8 == s / 8 && i < 64 {
        mask |= 1 << i;
        if occupancy.0 & (1 << i) != 0 {
            break;
        }
        i += 1;
    }
    i = s;
    while i / 8 == s / 8 && i >= 0 {
        mask |= 1 << i;
        if occupancy.0 & (1 << i) != 0 {
            break;
        }
        i -= 1;
    }
    i = s;
    // cover the columns
    while i < 64 {
        mask |= 1 << i;
        if occupancy.0 & (1 << i) != 0 {
            break;
        }
        i += 8;
    }
    i = s;
    while i >= 0 {
        mask |= 1 << i;
        if occupancy.0 & (1 << i) != 0 {
            break;
        }
        i -= 8;
    }
    // our current logic adds the starting square, so we just remove it
    mask & !(1 << s)
}

/// returns a mask used for indexing bishop attack patterns. The mask contains all movable squares
/// from starting square whith a bishop, except the border squares.
pub fn create_bishop_mask(square: Square) -> u64 {
    let mut mask: u64 = 0;

    let mut i = square.0;

    // goes into ++ direction until it hits the edge of the board
    while i % 8 < 7 && i / 8 < 7 {
        mask |= 1 << i;
        i += 9;
    }
    i = square.0;
    // goes into -- direction until it hits the edge of the board
    while i % 8 > 0 && i / 8 > 0 {
        mask |= 1 << i;
        i -= 9;
    }
    i = square.0;
    // goes into -+ direction until it hits the edge of the board
    while i % 8 < 7 && i / 8 > 0 {
        mask |= 1 << i;
        i -= 7;
    }
    i = square.0;
    // goes into +- direction until it hits the edge of the board
    while i % 8 > 0 && i / 8 < 7 {
        mask |= 1 << i;
        i += 7;
    }
    // our current logic adds the starting square, so we just remove it
    mask & !(1 << square.0)
}
/// returns the bitboard pattern for all possible rook moves with given occupancy starting at square
pub fn create_bishop_attack_pattern(square: Square, occupancy: Occupancy) -> u64 {
    let mut mask: u64 = 0;
    let s = square.0 as i8;
    let mut i = s;

    // goes into ++ direction until it hits the edge of the board
    while i < 64 {
        mask |= 1 << i;
        if occupancy.0 & (1 << i) != 0 || i % 8 == 7 {
            break;
        }
        i += 9;
    }
    i = s;
    // goes into -- direction until it hits the edge of the board
    while i >= 0 {
        mask |= 1 << i;
        if occupancy.0 & (1 << i) != 0 || i % 8 == 0 {
            break;
        }
        i -= 9;
    }
    i = s;
    // goes into -+ direction until it hits the edge of the board
    while i >= 0 {
        mask |= 1 << i;
        if occupancy.0 & (1 << i) != 0 || i % 8 == 7 {
            break;
        }
        i -= 7;
    }
    i = s;
    // goes into +- direction until it hits the edge of the board
    while i < 64 {
        mask |= 1 << i;
        if occupancy.0 & (1 << i) != 0 || i % 8 == 0 {
            break;
        }
        i += 7;
    }
    // our current logic adds the starting square, so we just remove it
    mask & !(1 << square.0)
}

#[cfg(test)]
mod test {
    use super::*;

    fn squares_from_bitboard(bb: u64) -> Vec<String> {
        let mut squares = Vec::new();
        for i in 0..64 {
            if bb & (1 << i) != 0 {
                squares.push(Square(i).to_string());
            }
        }
        squares
    }

    fn check_bit_board_pattern(expected: u64, computed: u64) {
        assert_eq!(
            computed,
            expected,
            "assert failed:\nexpected: {:>64b}\ngot:      {:>64b}\nerror:    {:>64b}\nsquares:  {:?}",
            expected,
            computed,
            computed ^ expected,
            squares_from_bitboard(computed ^ expected),
        )
    }

    #[test]
    fn test_create_bishop_mask() {
        // put the bishop on d3;
        let s = Square::from_file_rank(File::D, Rank::Four);
        let m = create_bishop_mask(s);
        let expected = E3.as_bbs() | F2.as_bbs() | C3.as_bbs() | B2.as_bbs() | C5.as_bbs() | B6.as_bbs() | E5.as_bbs() | F6.as_bbs() | G7.as_bbs();
        check_bit_board_pattern(expected, m);
    }

    #[test]
    fn test_create_rook_mask() {
        // put the bishop on d3;
        let s = Square::from_file_rank(File::D, Rank::Four);
        let m = create_rook_mask(s);

        let expected =
            D5.as_bbs() | D6.as_bbs() | D7.as_bbs() | D3.as_bbs() | D2.as_bbs() | C4.as_bbs() | B4.as_bbs() | E4.as_bbs() | F4.as_bbs() | G4.as_bbs();
        check_bit_board_pattern(expected, m);
    }

    #[test]
    fn test_create_bishop_attack_pattern_no_occupants() {
        // put the bishop on d3;
        let s = Square::from_file_rank(File::D, Rank::Four);
        let m = create_bishop_attack_pattern(s, Occupancy(0));

        let expected = E3.as_bbs()
            | F2.as_bbs()
            | G1.as_bbs()
            | C3.as_bbs()
            | B2.as_bbs()
            | A1.as_bbs()
            | C5.as_bbs()
            | B6.as_bbs()
            | A7.as_bbs()
            | E5.as_bbs()
            | F6.as_bbs()
            | G7.as_bbs()
            | H8.as_bbs();

        check_bit_board_pattern(expected, m);
    }
    #[test]
    fn test_create_bishop_attack_pattern_with_occupants() {
        // put the bishop on d3;
        let s = Square::from_file_rank(File::D, Rank::Four);
        let occupancy = F2.as_bbs() | A1.as_bbs() | C5.as_bbs() | F6.as_bbs();
        let m = create_bishop_attack_pattern(s, Occupancy(occupancy));

        let expected = E3.as_bbs() | F2.as_bbs() | C3.as_bbs() | B2.as_bbs() | A1.as_bbs() | C5.as_bbs() | E5.as_bbs() | F6.as_bbs();

        check_bit_board_pattern(expected, m);
    }

    #[test]
    fn test_create_rook_attack_pattern_no_occupants() {
        // put the bishop on d3;
        let s = Square::from_file_rank(File::D, Rank::Four);
        let m = create_rook_attack_pattern(s, Occupancy(0));
        let expected = D5.as_bbs()
            | D6.as_bbs()
            | D7.as_bbs()
            | D8.as_bbs()
            | D3.as_bbs()
            | D2.as_bbs()
            | D1.as_bbs()
            | C4.as_bbs()
            | B4.as_bbs()
            | A4.as_bbs()
            | E4.as_bbs()
            | F4.as_bbs()
            | G4.as_bbs()
            | H4.as_bbs();
        check_bit_board_pattern(expected, m);
    }
    #[test]
    fn test_create_rook_attack_pattern_with_occupants() {
        // put the bishop on d3;
        let s = Square::from_file_rank(File::D, Rank::Four);
        let occupancy = D7.as_bbs() | D3.as_bbs() | A4.as_bbs() | F4.as_bbs() | H4.as_bbs() | H8.as_bbs();
        let m = create_rook_attack_pattern(s, Occupancy(occupancy));
        let expected = D5.as_bbs() | D6.as_bbs() | D7.as_bbs() | D3.as_bbs() | C4.as_bbs() | B4.as_bbs() | A4.as_bbs() | E4.as_bbs() | F4.as_bbs();
        check_bit_board_pattern(expected, m);
    }
}
