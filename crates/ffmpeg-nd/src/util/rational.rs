use ffi::*;

#[derive(Default)]
pub struct Rational {
    pub num: u8,
    pub den: u8,
}

impl Rational {
    pub fn new(num: u8, den: u8) -> Self {
        Self { num, den }
    }
}

impl Into<AVRational> for Rational {
    fn into(self) -> AVRational {
        AVRational {
            num: self.num.into(),
            den: self.den.into(),
        }
    }
}

impl From<u8> for Rational {
    fn from(value: u8) -> Self {
        Self { num: value, den: 1 }
    }
}

impl From<[u8; 2]> for Rational {
    fn from(value: [u8; 2]) -> Self {
        Self { num: value[0], den: value[1] }
    }
}
