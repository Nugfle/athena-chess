use super::move_logic::*;
use super::square::Square;
use crate::engine::game::occupancy::Occupancy;
use rand::{self, random};
use std::usize;

/// the density with which the arrays will be paced. Increasing this will result in more sparsely
/// populated arrays but faster times for finding magic numbers
pub const H: u32 = 3;

#[derive(Debug, Clone)]
pub struct AttackMagic {
    pub mask: u64,
    // the magic number is unique to each mask and ensures the bijective property of our hash
    // function
    pub magic_number: u64,
    // the amount of bits used to form the address for the attac_pattern array
    pub shift: u8,
    // holds all possible attack patterns. The position in the array is determined by the hash of
    // the Occupancie. So we get a direct Mapping from currently occupied squares and our current
    // square, to all available moves.
    pub attack_patterns: Vec<u64>,
}
impl Default for AttackMagic {
    fn default() -> Self {
        Self {
            mask: 0,
            magic_number: 0,
            shift: 0,
            attack_patterns: Vec::new(),
        }
    }
}
/// creates all possbile Occupancy scenarios from the mask used for finding blockings
fn occupancies_from_mask(mask: u64) -> Vec<Occupancy> {
    let size = 2_usize.pow(mask.count_ones());
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
fn find_valid_magic_number(mask: u64, arr_size: usize, occupancies: &Vec<Occupancy>) -> u64 {
    // the shift is used to select the appropriate amount of msbs for a given array size to index
    // into.
    let shift = 64 - (arr_size as f32).log2().ceil() as u8;

    let mut magic_num;
    // we loop until we find a working number. As soon as we detect a colision we start again. This
    // could probably be optimized with multithreading for better performance.
    'outer: loop {
        magic_num = random();
        let mut arr = vec![false; arr_size];
        for occ in occupancies {
            if arr[occ.hash(mask, magic_num, shift)] {
                continue 'outer;
            }
            arr[occ.hash(mask, magic_num, shift)] = true;
        }
        return magic_num;
    }
}

impl AttackMagic {
    pub fn create_attack_magic_rook(square: Square) -> Self {
        let mask = create_rook_mask(square);
        let shift = (mask.count_ones() + H) as u8;
        let len = 2_usize.pow(shift as u32);
        let occupancies = occupancies_from_mask(mask);
        let magic_number = find_valid_magic_number(mask, len, &occupancies);
        let mut attack_patterns = vec![0; len];
        for occ in occupancies {
            attack_patterns[occ.hash(mask, magic_number, shift)] = create_rook_attack_pattern(square, occ);
        }

        Self {
            mask,
            magic_number,
            shift,
            attack_patterns,
        }
    }
    pub fn create_attack_magic_bishop(square: Square) -> Self {
        let mask = create_bishop_mask(square);
        let shift = (mask.count_ones() + H) as u8;
        let len = 2_usize.pow(shift as u32);
        let occupancies = occupancies_from_mask(mask);
        let magic_number = find_valid_magic_number(mask, len, &occupancies);
        let mut attack_patterns = vec![0; len];
        for occ in occupancies {
            attack_patterns[occ.hash(mask, magic_number, shift)] = create_bishop_attack_pattern(square, occ);
        }

        Self {
            mask,
            magic_number,
            shift,
            attack_patterns,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::engine::game::square::*;

    #[test]
    fn test_find_valid_magic_num() {
        let mask = create_rook_mask(E1);
        let o = occupancies_from_mask(mask);
        let num = find_valid_magic_number(mask, 2_usize.pow(mask.count_ones() + H), &o);
        panic!("found magic number: {}", num);
    }
}
