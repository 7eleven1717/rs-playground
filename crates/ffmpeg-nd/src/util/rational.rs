use ffi::*;

#[derive(Default)]
pub struct Rational {
    pub num: u32,
    pub den: u32,
}

impl Rational {
    pub fn new(num: u32, den: u32) -> Self {
        Self { num, den }
    }
}

impl Into<AVRational> for Rational {
    fn into(self) -> AVRational {
        AVRational {
            num: self.num.try_into().unwrap(),
            den: self.den.try_into().unwrap(),
        }
    }
}

impl Into<AVRational> for &Rational {
    fn into(self) -> AVRational {
        // TODO: May be overflowed.
        AVRational {
            num: self.num.try_into().unwrap(),
            den: self.den.try_into().unwrap(),
        }
    }
}

impl From<AVRational> for Rational {
    fn from(av_rational: AVRational) -> Self {
        Self {
            num: av_rational.num.try_into().unwrap(),
            den: av_rational.den.try_into().unwrap(),
        }
    }
}

impl From<u32> for Rational {
    fn from(num: u32) -> Self {
        Self { num, den: 1 }
    }
}

impl From<[u32; 2]> for Rational {
    fn from([num, den]: [u32; 2]) -> Self {
        Self { num, den }
    }
}
