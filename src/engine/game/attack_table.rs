use super::magic::attack_magic::AttackMagic;
use super::square::Square;
use crate::engine::game::occupancy::Occupancy;
use std::usize;

/// holds on to all the AttackMagics for all possible squares for rook and bishop
/// N is the size for the Rook attack-pattern array and M for the bishop.
/// N and M are therefore proportional to the memory usage by the factor of 64.
/// Lower N and M can increase Performance but it might take longer to create the Attack Tables.
pub struct AttackTables {
    pub rook_tables: [AttackMagic; 64],
    pub bishop_tables: [AttackMagic; 64],
    pub knight_table: [u64; 64],
    pub white_pawn_table: [u64; 56],
    pub black_pawn_table: [u64; 56],
}

impl AttackTables {
    pub fn create_tables() -> Self {
        let mut bishop_tables: [AttackMagic; 64] = core::array::from_fn(|_| AttackMagic::default());
        let mut rook_tables: [AttackMagic; 64] = core::array::from_fn(|_| AttackMagic::default());
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
        attack_magic.attack_patterns[occupancy.hash(attack_magic.mask, attack_magic.magic_number, attack_magic.shift)]
    }
    pub fn get_attack_pattern_bishop(&self, square: Square, occupancy: Occupancy) -> u64 {
        let attack_magic = &self.bishop_tables[square.0 as usize];
        // we need to
        attack_magic.attack_patterns[occupancy.hash(attack_magic.mask, attack_magic.magic_number, attack_magic.shift)]
    }
    pub fn get_attack_pattern_queen(&self, square: Square, occupancy: Occupancy) -> u64 {
        self.get_attack_pattern_rook(square, occupancy) | self.get_attack_pattern_bishop(square, occupancy)
    }
}
