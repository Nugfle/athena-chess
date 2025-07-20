use super::bit_board::Square;
use super::occupancy::Occupancy;

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
    while i % 8 > 0 {
        mask |= 1 << i - 1;
        i -= 8;
    }
    mask
}
pub fn create_rook_attack_pattern(square: Square, occupancy: Occupancy) -> u64 {
    let mut mask: u64 = 0;
    let mut i = square.0;
    // cover the rows
    while i % 8 < 7 {
        mask |= 1 << i;
        if occupancy.0 & 1 << i != 1 {
            break;
        }
        i += 1;
    }
    i = square.0;
    while i % 8 > 0 {
        mask |= 1 << i;
        if occupancy.0 & 1 << i != 1 {
            break;
        }
        i -= 1;
    }
    i = square.0;
    // cover the columns
    while i / 8 < 7 {
        mask |= 1 << i;
        if occupancy.0 & 1 << i != 1 {
            break;
        }
        i += 8;
    }
    i = square.0;
    while i % 8 > 0 {
        mask |= 1 << i - 1;
        if occupancy.0 & 1 << i != 1 {
            break;
        }
        i -= 8;
    }
    mask
}
pub fn create_bishop_mask(square: Square) -> u64 {
    let mut mask: u64 = 0;
    let mut i = square.0;
    // first and third quadrant
    while i % 8 < 7 && i / 8 < 7 {
        mask |= 1 << i;
        i += 9;
    }
    while i % 8 > 0 && i / 8 > 0 {
        mask |= 1 << i;
        i -= 9;
    }
    // second and fourth quandrant
    while i % 8 < 7 && i / 8 > 0 {
        mask |= 1 << i;
        i -= 7;
    }
    while i % 8 > 0 && i / 8 < 7 {
        mask |= 1 << i;
        i += 7;
    }
    mask
}
pub fn create_bishop_attack_pattern(square: Square, occupancy: Occupancy) -> u64 {
    let mut mask: u64 = 0;
    let mut i = square.0;
    // first and third quadrant
    while i % 8 < 7 && i / 8 < 7 {
        mask |= 1 << i;
        if occupancy.0 & 1 << i != 1 {
            break;
        }
        i += 9;
    }
    while i % 8 > 0 && i / 8 > 0 {
        mask |= 1 << i;
        if occupancy.0 & 1 << i != 1 {
            break;
        }
        i -= 9;
    }
    // second and fourth quandrant
    while i % 8 < 7 && i / 8 > 0 {
        mask |= 1 << i;
        if occupancy.0 & 1 << i != 1 {
            break;
        }
        i -= 7;
    }
    while i % 8 > 0 && i / 8 < 7 {
        mask |= 1 << i;
        if occupancy.0 & 1 << i != 1 {
            break;
        }
        i += 7;
    }
    mask
}
pub fn find_valid_magic_number(mask: u64, arr_size: usize) -> u64 {
    todo!()
}
