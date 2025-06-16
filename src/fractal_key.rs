use crate::complex::ComplexDecimal;
use rand::Rng;

/// Holds a key of 96 * 4 + 8 bits
/// 96 for each number, 4 bits for signs, and 4 bits for other use
/// Total of 392 bits
/// Leaves 120 bits for maybe another 96 bit number and then 24 bits (-1 for sign?) to get a neat 512 key?
pub struct FractalKey {
    data: Vec<u8>,
}

impl FractalKey {
    // pub fn new(seed: &str) -> Self {
    //     Self {
    //         data: seed.to_bytes(),
    //     }
    // }
    pub fn new() -> Self {
        let mut data = Vec::with_capacity(64);

        let mut rng = rand::rng();
        for d in data.iter_mut() {
            *d = rng.random::<u8>()
        }

        Self { data }
    }

    pub fn get_c(&self) -> ComplexDecimal {
        // for now just the first 24 bytes are the main numbers of c,
        // and then the next 2 bytes are the negative flags.
        // TODO make flags bitvec and not need their own bytes
        ComplexDecimal::from_u8s(&self.data[0..27])
    }

    pub fn get_z0(&self) -> ComplexDecimal {
        // for now just the first 24 bytes are the main numbers of c,
        // and then the next 2 bytes are the negative flags.
        // TODO make flags bitvec and not need their own bytes
        ComplexDecimal::from_u8s(&self.data[27..54])
    }

    pub fn get_extra(&self) -> u8 {
        self.data[55]
    }
}
