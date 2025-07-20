use std::usize;

/// a representation of the board where each bit in the u64 represents the square on the board and
/// whether it is occupied. This makes checking for blocking pieces as easy as applying a mask to
/// the Occupancy and voila, you get all the squares with blocking pieces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Occupancy(pub u64);

impl Occupancy {
    // TODO: optimize the shit out of this for a as dense as possible bijective mapping
    pub const fn hash(&self, mask: u64, magic_number: u64, shift: u8, max: usize) -> usize {
        // we mask off only the relevant squares (so f.e. for a rook that would be the horizontal and
        // vertical line it is on), this is the first importaint step as it reduces complexity from
        // 2^64 down to 2^n where n is the number of relevant squares which is way more manageble.
        // We then try to create a as dense as possible bijection from the 2^n occupancy patterns
        // to an usize which can serve as an Index into an Attack Pattern Array. the mod N ensures
        // we are within the size of the array.
        ((self.0 & mask).wrapping_mul(magic_number) >> shift) as usize % max
    }
}
