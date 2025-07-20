use super::occupancy::Occupancy;
use super::square::Square;

/// computes the amount of bits neccesary to address a slice of memory of size
pub fn bits_to_address(n: usize) -> u8 {
    // to address an array of size n we need at least ceil(log_2(n)) bits.
    // But we can avoid the logarithm because we just want to know for what m, 2^m = n
    // and 2^m is equivalent to just left shifting 1 by m.
    let mut m: u8 = 0;
    loop {
        if 1_usize << m >= n {
            return m;
        }
        m += 1;
    }
}
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
    mask & !(1 << square.0)
}
pub fn create_rook_attack_pattern(square: Square, occupancy: Occupancy) -> u64 {
    let mut mask: u64 = 0;
    // is between 0 and 63
    let mut i = square.0 as i8;
    // cover the row
    while i % 8 <= 7 && i > 0 && i / 8 == square.0 as i8 / 8 {
        mask |= 1 << i;
        if occupancy.0 & (1 << i) == 1 {
            break;
        }
        i += 1;
    }
    i = square.0 as i8;
    while i % 8 >= 0 && i > 0 && i / 8 == square.0 as i8 / 8 {
        mask |= 1 << i;
        if occupancy.0 & (1 << i) == 1 {
            break;
        }
        i -= 1;
    }
    i = square.0 as i8;
    // cover the column
    while i / 8 <= 7 && i > 0 {
        mask |= 1 << i;
        if occupancy.0 & (1 << i) == 1 {
            break;
        }
        i += 8;
    }
    i = square.0 as i8;
    while i / 8 >= 0 && i > 0 {
        mask |= 1 << i;
        if occupancy.0 & (1 << i) == 1 {
            break;
        }
        i -= 8;
    }
    mask & !(1 << square.0)
}
pub fn create_bishop_mask(square: Square) -> u64 {
    let mut mask: u64 = 0;
    let mut i = square.0;
    // first and third quadrant
    while i % 8 < 7 && i / 8 < 7 {
        mask |= 1 << i;
        i += 9;
    }
    let mut i = square.0;
    while i % 8 > 0 && i / 8 > 0 {
        mask |= 1 << i;
        i -= 9;
    }
    let mut i = square.0;
    // second and fourth quandrant
    while i % 8 < 7 && i / 8 > 0 {
        mask |= 1 << i;
        i -= 7;
    }
    let mut i = square.0;
    while i % 8 > 0 && i / 8 < 7 {
        mask |= 1 << i;
        i += 7;
    }
    mask & !(1 << square.0)
}
pub fn create_bishop_attack_pattern(square: Square, occupancy: Occupancy) -> u64 {
    let mut mask: u64 = 0;
    let mut i = square.0 as i8;
    // first and third quadrant
    while i % 8 <= 7 && i / 8 <= 7 && i > 0 {
        mask |= 1 << i;
        if occupancy.0 & (1 << i) == 1 {
            break;
        }
        i += 9;
    }
    i = square.0 as i8;
    while i % 8 >= 0 && i / 8 >= 0 && i > 0 {
        mask |= 1 << i;
        if occupancy.0 & (1 << i) == 1 {
            break;
        }
        i -= 9;
    }
    i = square.0 as i8;
    // second and fourth quandrant
    while i % 8 <= 7 && i / 8 >= 0 && i > 0 {
        mask |= 1 << i;
        if occupancy.0 & (1 << i) == 1 {
            break;
        }
        i -= 7;
    }
    i = square.0 as i8;
    while i % 8 >= 0 && i / 8 <= 7 && i > 0 {
        mask |= 1 << i;
        if occupancy.0 & (1 << i) == 1 {
            break;
        }
        i += 7;
    }
    mask & !(1 << square.0)
}
pub fn find_valid_magic_number(mask: u64, arr_size: usize) -> u64 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::engine::game::square::*;
    pub fn squares_from_bitboard(bb: u64) -> Vec<String> {
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
}
