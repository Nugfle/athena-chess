use log::info;
use rand::{self, random};

use super::move_logic::*;
use crate::game::BoardMask;
use crate::game::board::Occupancy;
use crate::game::board::square::Square;

/// the density with which the arrays will be paced. Increasing this will result in more sparsely
/// populated arrays but faster times for finding magic numbers
pub const H: u32 = 1;

#[derive(Debug, Clone, Default)]
pub struct AttackMagic {
    pub mask: BoardMask,
    // the magic number is unique to each mask and ensures the bijective property of our hash
    // function
    pub magic_number: u64,
    // the amount of bits used to form the address for the attac_pattern array
    pub shift: u8,
    // holds all possible attack patterns. The position in the array is determined by the hash of
    // the Occupancie. So we get a direct Mapping from currently occupied squares and our current
    // square, to all available moves.
    pub attack_patterns: Vec<BoardMask>,
}

impl AttackMagic {
    /// creates magic numbers and computes the attack patterns for the given square
    pub fn create_attack_magic_rook(square: Square) -> Self {
        info!("creating rook magic for {square}");
        let mask = create_rook_mask(square);

        let required_bits = (mask.0.count_ones() + H) as u8;
        let len = 2_usize.pow(required_bits as u32);
        let shift = 64 - required_bits;

        let occupancies = occupancies_from_mask(mask);
        let magic_number = find_valid_magic_number(mask, len, &occupancies);

        let mut attack_patterns: Vec<BoardMask> = vec![BoardMask(0); len];
        occupancies
            .iter()
            .map(|occ| (occ.hash(mask, magic_number, shift), create_rook_attack_pattern(square, *occ)))
            .for_each(|(h, t)| attack_patterns[h] = t);

        Self {
            mask,
            magic_number,
            shift,
            attack_patterns,
        }
    }

    /// creates magic numbers and computes the attack patterns for the given square
    pub fn create_attack_magic_bishop(square: Square) -> Self {
        info!("creating bishop magic for {square}");
        let mask = create_bishop_mask(square);

        let required_bits = (mask.0.count_ones() + H) as u8;
        let len = 2_usize.pow(required_bits as u32);
        let shift = 64 - required_bits;

        let occupancies = occupancies_from_mask(mask);
        let magic_number = find_valid_magic_number(mask, len, &occupancies);

        let mut attack_patterns: Vec<BoardMask> = vec![BoardMask(0); len];
        occupancies
            .iter()
            .map(|occ| (occ.hash(mask, magic_number, shift), create_bishop_attack_pattern(square, *occ)))
            .for_each(|(h, t)| attack_patterns[h] = t);

        Self {
            mask,
            magic_number,
            shift,
            attack_patterns,
        }
    }
}

impl Occupancy {
    pub const fn hash(&self, mask: BoardMask, magic_number: u64, shift: u8) -> usize {
        // we mask off only the relevant squares (so f.e. for a rook that would be the horizontal and
        // vertical line it is on), this is the first importaint step as it reduces complexity from
        // 2^64 down to 2^n where n is the number of relevant squares which is way more manageble.
        // We then try to create a as dense as possible bijection from the 2^n occupancy patterns
        // to an usize which can serve as an Index into an Attack Pattern Array.
        ((self.0 & mask.0).wrapping_mul(magic_number) >> shift) as usize
    }
}

/// creates all possbile Occupancy scenarios from the mask used for finding blockings
fn occupancies_from_mask(mask: BoardMask) -> Vec<Occupancy> {
    let size = 2_usize.pow(mask.0.count_ones());
    let mut v = Vec::with_capacity(size);

    v.push(Occupancy(0));

    // we go through all the bits in the mask. If the bit is set we effectively duplicate our
    // current Vector with the newly found bit set.
    for i in 0..64 {
        if mask.contains(Square::new(i).unwrap()) {
            v.append(&mut v.iter().map(|o| o.with_square(Square::new(i).unwrap())).collect());
        };
    }
    v
}

/// finds a valid magic number so the hash over all possible occupancies for a given mask is
/// bijective. This method uses try and error and is highly resource intensive. If possible should
/// use pre-computed magic values and load them from disk
fn find_valid_magic_number(mask: BoardMask, arr_size: usize, occupancies: &Vec<Occupancy>) -> u64 {
    // the shift is used to select the appropriate amount of msbs for a given array size to index
    // into.
    let shift = 64 - (arr_size as f32).log2().ceil() as u8;
    let mut arr = vec![false; arr_size];

    let mut magic_num;
    // we loop until we find a working number. As soon as we detect a colision we start again. This
    // could probably be optimized with multithreading for better performance.
    'outer: loop {
        magic_num = random();
        for occ in occupancies {
            if arr[occ.hash(mask, magic_num, shift)] {
                arr.fill(false);
                continue 'outer;
            }
            arr[occ.hash(mask, magic_num, shift)] = true;
        }
        return magic_num;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::board::square::*;

    #[test]
    fn test_find_valid_magic_num() {
        let mask = create_rook_mask(E1);
        let o = occupancies_from_mask(mask);
        find_valid_magic_number(mask, 2_usize.pow(mask.0.count_ones() + H), &o);
    }
}
