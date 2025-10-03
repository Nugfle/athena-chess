//! Magic bitboard implementation for fast sliding piece move generation.
//!
//! This module implements the magic bitboard technique, which provides:
//! - O(1) move generation for sliding pieces (rooks, bishops, queens)
//! - Compact storage of pre-calculated attack patterns
//! - Perfect hashing of piece-blocking configurations
//!
//! # Magic Bitboards
//!
//! Magic bitboards use perfect hashing to map board occupancy patterns
//! to pre-calculated attack patterns. The technique involves:
//!
//! 1. Creating masks for relevant blocking squares
//! 2. Finding "magic" numbers that create perfect hash functions
//! 3. Pre-calculating all possible attack patterns
//! 4. Using the magic hash to look up patterns during move generation
//!
//! # Performance
//!
//! - Move generation: O(1) (two array lookups)
//! - Memory usage: Configurable via sparseness factor H
//! - Initialization: One-time cost, can be cached to disk
//!
//! # Examples
//!
//! ```rust
//! use athena_core::game::attack_tables::{AttackMagic, Square};
//!
//! // Create magic bitboard for a rook
//! let rook_magic = AttackMagic::create_attack_magic_rook(Square::E4);
//!
//! // Get attack pattern for current board state
//! let occupancy = current_board.get_occupancy();
//! let attacks = rook_magic.attack_patterns[occupancy.hash(
//!     rook_magic.mask,
//!     rook_magic.magic_number,
//!     rook_magic.shift
//! )];
//! ```

use log::info;
use rand::{self, random};
use serde::{Deserialize, Serialize};

use super::move_logic::*;
use crate::game::BoardMask;
use crate::game::board::Occupancy;
use crate::game::board::square::Square;

/// Sparseness factor for attack pattern arrays.
///
/// Higher values result in:
/// - Sparser arrays (more memory usage)
/// - Faster magic number generation
/// - Potentially faster lookups
///
/// Lower values provide:
/// - More compact arrays
/// - Slower magic number generation
/// - Potentially slower lookups
pub const H: u32 = 1;

/// Magic bitboard data structure for a single piece type and square.
///
/// This struct contains all the data needed for O(1) attack pattern lookup:
/// - Pre-calculated attack patterns for all possible occupancies
/// - Magic number for perfect hashing
/// - Mask of relevant blocking squares
/// - Shift value for hash calculation
///
/// # Fields
///
/// * `mask` - Bitboard of squares that can block the piece's movement
/// * `magic_number` - Number that creates perfect hash function
/// * `shift` - Number of bits to right-shift for final hash
/// * `attack_patterns` - Pre-calculated patterns indexed by occupancy hash
///
/// # Examples
///
/// ```rust
/// use athena_core::game::attack_tables::AttackMagic;
///
/// // Create magic bitboard for bishop on E4
/// let bishop_magic = AttackMagic::create_attack_magic_bishop(Square::E4);
///
/// // Look up attack pattern
/// let hash = occupancy.hash(bishop_magic.mask,
///                          bishop_magic.magic_number,
///                          bishop_magic.shift);
/// let attacks = bishop_magic.attack_patterns[hash];
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
        // vertical line it is on), this is the first important step as it reduces complexity from
        // 2^64 down to 2^n where n is the number of relevant squares which is way more manageble.
        // We then try to create a as dense as possible linear bijection from the 2^n occupancy patterns
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

#[cfg(feature = "magic_test")]
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
