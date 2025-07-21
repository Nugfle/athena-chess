use super::move_logic::*;
use super::square::Square;
use crate::engine::game::occupancy::Occupancy;
use std::usize;

#[derive(Debug, Clone)]
pub struct AttackMagic<const N: usize> {
    pub mask: u64,
    // the magic number is unique to each mask and ensures the bijective property of our hash
    // function
    pub magic_number: u64,
    // the amount of bits used to form the address for the attac_pattern array
    pub shift: u8,
    // holds all possible attack patterns. The position in the array is determined by the hash of
    // the Occupancie. So we get a direct Mapping from currently occupied squares and our current
    // square, to all available moves.
    pub attack_pattern: [u64; N],
}
impl<const N: usize> Default for AttackMagic<N> {
    fn default() -> Self {
        Self {
            mask: 0,
            magic_number: 0,
            shift: 0,
            attack_pattern: [0; N],
        }
    }
}

/// holds on to all the AttackMagics for all possible squares for rook and bishop
/// N is the size for the Rook attack-pattern array and M for the bishop.
/// N and M are therefore proportional to the memory usage by the factor of 64.
/// Lower N and M can increase Performance but it might take longer to create the Attack Tables.
pub struct AttackTables<const N: usize, const M: usize> {
    pub rook_tables: [AttackMagic<N>; 64],
    pub bishop_tables: [AttackMagic<M>; 64],
    pub knight_table: [u64; 64],
    pub white_pawn_table: [u64; 56],
    pub black_pawn_table: [u64; 56],
}

impl<const M: usize, const N: usize> AttackTables<M, N> {
    pub fn create_tables() -> Self {
        let mut bishop_tables: [AttackMagic<N>; 64] = core::array::from_fn(|_| AttackMagic::default());
        let mut rook_tables: [AttackMagic<M>; 64] = core::array::from_fn(|_| AttackMagic::default());
        for i in 0..64 {
            bishop_tables[i as usize] = AttackMagic::create_attack_magic_bishop(Square(i));
            rook_tables[i as usize] = AttackMagic::create_attack_magic_rook(Square(i));
        }

        Self {
            rook_tables,
            bishop_tables,
            knight_table: todo!(),
            white_pawn_table: todo!(),
            black_pawn_table: todo!(),
        }
    }
    pub fn get_attack_pattern_rook(&self, square: Square, occupancy: Occupancy) -> u64 {
        let attack_magic = &self.rook_tables[square.0 as usize];
        attack_magic.attack_pattern[occupancy.hash(attack_magic.mask, attack_magic.magic_number, attack_magic.shift, M)]
    }
    pub fn get_attack_pattern_bishop(&self, square: Square, occupancy: Occupancy) -> u64 {
        let attack_magic = &self.bishop_tables[square.0 as usize];
        // we need to
        attack_magic.attack_pattern[occupancy.hash(attack_magic.mask, attack_magic.magic_number, attack_magic.shift, M)]
    }
    pub fn get_attack_pattern_queen(&self, square: Square, occupancy: Occupancy) -> u64 {
        self.get_attack_pattern_rook(square, occupancy) | self.get_attack_pattern_bishop(square, occupancy)
    }
}

/// computes the amount of bits neccesary to address a slice of memory of size n
pub fn bits_to_address(n: usize) -> u8 {
    f32::log2(n as f32).ceil() as u8
}

impl<const N: usize> AttackMagic<N> {
    pub fn create_attack_magic_rook(square: Square) -> Self {
        let mask = create_rook_mask(square);
        let shift = 64 - bits_to_address(N);
        let magic_number = find_valid_magic_number(mask, N);

        Self {
            mask,
            magic_number,
            shift,
            attack_pattern: todo!(),
        }
    }
    pub fn create_attack_magic_bishop(square: Square) -> Self {
        let mask = create_bishop_mask(square);
        let magic_number = find_valid_magic_number(mask, N);
        let shift = 64 - bits_to_address(N);
        Self {
            mask,
            magic_number,
            shift,
            attack_pattern: todo!(),
        }
    }
}
