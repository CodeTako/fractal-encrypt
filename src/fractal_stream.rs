use crate::{complex::ComplexDecimal, fractal_key::FractalKey};
use rust_decimal::prelude::*;
use std::collections::VecDeque;

pub struct FractalStream {
    c: ComplexDecimal,
    z: ComplexDecimal,
    iter_count: u64,
    upcoming: VecDeque<u8>,
}

impl FractalStream {
    pub fn new(key: FractalKey) -> Self {
        let c = key.get_c();
        let z = key.get_z0();

        Self {
            c,
            z,
            iter_count: 0,
            upcoming: VecDeque::new(),
        }
    }

    pub fn next(&mut self) -> u8 {
        let next_byte = self.upcoming.pop_front().unwrap();

        if self.upcoming.len() < 8 {
            self.next_iteration();
        }

        next_byte
    }

    pub fn get_iter_count(&self) -> u64 {
        self.iter_count
    }

    fn next_iteration(&mut self) {
        self.iter_count += 1;

        let mut new_z = self.z.mult(&self.z).add(&self.c);
        println!("{} -> {}", self.iter_count, new_z);

        if new_z.square_modulus() >= dec![4] {
            new_z = new_z.scale(dec![0.5]);
            println!("  Scaled -> {}", new_z);
        }

        self.z = new_z;
        for byte in self.z.bytes() {
            self.upcoming.push_back(byte);
        }
    }
}
