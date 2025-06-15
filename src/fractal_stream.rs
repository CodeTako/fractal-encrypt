use rust_decimal::Decimal;

struct FractalStream {
    c_x: Decimal,
    c_y: Decimal,
    z_a: Decimal,
    z_b: Decimal,
    iter_count: u64,
    upcoming: Queue??<u8>,
}

impl FractalStream {
    pub fn new(key: FractalKey) -> Self {
        c_x, c_y = key.get_c();
        z_a, z_b = key.get_z0();

        Self {
            c_x,
            c_y,
            z_a,
            z_b,
            iter_count: 0,
            upcoming: Queue::new(),
        }
    }

    pub fn next(&mut self) -> u8 {
        let next_byte = upcoming.pop();

        if upcoming.len() < 4 {
            self.next_iteration();
        }

        next_byte
    }

    pub fn get_iter_count(&self) {
        self.iter_count
    }

    fn next_iteration(&mut self) {}
}
