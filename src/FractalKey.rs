use rust_decimal::Decimal;

/// Holds a key of 96 * 4 + 8 bits
/// 96 for each number, 4 bits for signs, and 4 bits for other use
/// Total of 392 bits
/// Leaves 120 bits for maybe another 96 bit number and then 24 bits (-1 for sign?) to get a neat 512 key?
struct FractalKey {
    data: Vec<u8>,
}

impl FractalKey {
    pub fn new(seed: &str) -> Self {
        Self {
            data: seed.to_bytes(),
        }
    }

    pub fn get_c() -> (Decimal, Decimal) {}

    pub fn get_z0() -> (Decimal, Decimal) {}

    pub fn get_extra() -> u8 {}
}
