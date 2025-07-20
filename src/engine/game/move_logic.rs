use std::collections::HashSet;

use log::info;

use super::occupancy::Occupancy;
use super::square::Square;

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

    while i / 8 == s / 8 {
        mask |= 1 << i;
        if occupancy.0 & (1 << i) != 0 {
            break;
        }
        i += 1;
    }
    i = s;
    while i / 8 == s / 8 {
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

/// creates all possbile Occupancy scenarios
fn occupancies_from_mask(mask: u64) -> Vec<Occupancy> {
    let size = 2_usize.pow(mask.count_ones());
    info!("allocating {} bytes for mask: {}", size * 8, mask);
    let mut v = Vec::with_capacity(size);
    v.push(Occupancy(0));

    // we go through all the bits in the mask. If the bit is set we effectively duplicate our
    // current Vector with the newly found bit set.
    for i in 0..64 {
        if mask & (1 << i) != 0 {
            v.append(&mut v.iter().map(|o| Occupancy(o.0 | (1 << i))).collect());
        };
    }
    v
}

/// finds a valid magic number so the hash over all possible occupancies for a given mask is
/// bijective. This method uses try and error and is highly resource intensive. If possible should
/// use pre-computed magic values and load them from disk
pub fn find_valid_magic_number(mask: u64, arr_size: usize) -> u64 {
    let occupancies = occupancies_from_mask(mask);
    println!("occupancies: {}", occupancies.len());
    let mut magic_num = 0;

    // the shift is used to select the appropriate amount of msbs for a given array size to index
    // into.
    let shift = 64 - (arr_size as f32).log2().ceil() as u8;

    // we loop until we find a working number. As soon as we detect a colision we start again. This
    // could probably be optimized with multithreading for better performance.
    'outer: loop {
        magic_num += 1;
        let arr = vec![false; arr_size];
        for occ in &occupancies {
            if arr[occ.hash(mask, magic_num, shift, arr_size)] {
                continue 'outer;
            }
        }
        return magic_num;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::engine::game::{occupancy, square::*};
    #[test]
    fn test_find_valid_magic_num() {
        let mask = create_rook_mask(E1);
        let num = find_valid_magic_number(mask, 2_usize.pow(mask.count_ones()));
        panic!("found magic number: {}", num);
    }

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
