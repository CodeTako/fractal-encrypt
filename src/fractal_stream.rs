use crate::complex::ComplexDecimal;
use std::collections::VecDeque;

struct FractalStream {
    c: ComplexDecimal,
    z: ComplexDecimal,
    iter_count: u64,
    upcoming: VecDeque<u8>,
}

impl FractalStream {
    pub fn new(key: FractalKey) -> Self {
        c = key.get_c();
        z = key.get_z0();

        Self {
            c,
            z,
            iter_count: 0,
            upcoming: VecDeque::new(),
        }
    }

    pub fn next(&mut self) -> u8 {
        let next_byte = upcoming.pop_front();

        if upcoming.len() < 8 {
            self.next_iteration();
        }

        next_byte
    }

    pub fn get_iter_count(&self) {
        self.iter_count
    }

    fn next_iteration(&mut self) {
        let mut new_z = self.z.mult(self.z).add(self.c);

        if new_z.square_modulus >= 4 {
            new_z = new_z.scale(dec![0.5]);
        }

        self.z = new_z;
        self.iter_count += 1;
        for byte in new_z.bytes() {
            self.upcoming.push_back(byte);
        }
    }
}
