use rust_decimal::Decimal;

struct ComplexDecimal {
    real: Decimal,
    imaginary: Decimal,
}

impl ComplexDecimal {
    pub fn new(real: Decimal, imaginary: Decimal) -> Self {
        Self { real, imaginary }
    }

    pub fn add(self, other: &Self) -> Self {
        Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }

    pub fn mult(self, other: &Self) -> Self {
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
