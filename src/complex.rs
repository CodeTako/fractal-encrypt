use rust_decimal::Decimal;

const DEFAULT_SCALE: u32 = 1;

pub struct ComplexDecimal {
    real: Decimal,
    imaginary: Decimal,
}

impl ComplexDecimal {
    pub fn new(real: Decimal, imaginary: Decimal) -> Self {
        Self { real, imaginary }
    }

    pub fn from_u8s(data: &[u8]) -> Self {
        if data.len() != 26 {
            panic!(
                "Tried to create a ComplexDecimal with an incorrect number of bytes: {}",
                data.len()
            );
        }

        let lo_bytes = <[u8; 4]>::try_from(&data[0..4]).unwrap();
        let lo: u32 = u32::from_le_bytes(lo_bytes);

        let mid_bytes = <[u8; 4]>::try_from(&data[4..8]).unwrap();
        let mid: u32 = u32::from_le_bytes(mid_bytes);

        let hi_bytes = <[u8; 4]>::try_from(&data[8..12]).unwrap();
        let hi: u32 = u32::from_le_bytes(hi_bytes);

        let negative = data[25] == 1_u8;

        let real = Decimal::from_parts(lo, mid, hi, negative, DEFAULT_SCALE);

        let lo_bytes = <[u8; 4]>::try_from(&data[12..16]).unwrap();
        let lo: u32 = u32::from_le_bytes(lo_bytes);

        let mid_bytes = <[u8; 4]>::try_from(&data[16..20]).unwrap();
        let mid: u32 = u32::from_le_bytes(mid_bytes);

        let hi_bytes = <[u8; 4]>::try_from(&data[20..24]).unwrap();
        let hi: u32 = u32::from_le_bytes(hi_bytes);

        let negative = data[26] == 1_u8;

        let imaginary = Decimal::from_parts(lo, mid, hi, negative, DEFAULT_SCALE);

        Self { real, imaginary }
    }

    pub fn add(self, other: &Self) -> Self {
        Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }

    pub fn mult(&self, other: &Self) -> Self {
        Self {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real,
        }
    }

    pub fn square_modulus(&self) -> Decimal {
        self.real * self.real + self.imaginary + self.imaginary
    }

    pub fn scale(self, scalar: Decimal) -> Self {
        Self {
            real: self.real * scalar,
            imaginary: self.imaginary * scalar,
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        // TODO
        vec![]
    }
}
