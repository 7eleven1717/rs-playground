use crate::util::Rational;
use core::ops::Deref;

#[derive(Clone, Copy)]
pub struct Framerate(pub u8);

impl Default for Framerate {
    fn default() -> Self {
        Preset::default().into()
    }
}

impl Framerate {
    pub fn into_timebase(&self) -> Timebase {
        Timebase(self.0)
    }
}

impl Framerate {
    pub fn f24() -> Self {
        Preset::Fps24.into()
    }
    pub fn f30() -> Self {
        Preset::Fps30.into()
    }
    pub fn f60() -> Self {
        Preset::Fps60.into()
    }
    pub fn change_to_24(&mut self) {
        *self = Self::f24();
    }
    pub fn change_to_30(&mut self) {
        *self = Self::f30();
    }
    pub fn change_to_60(&mut self) {
        *self = Self::f60();
    }
}

impl Deref for Framerate {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<Rational> for Framerate {
    fn into(self) -> Rational {
        Rational::new(self.0.into(), 1)
    }
}

impl Into<Timebase> for Framerate {
    fn into(self) -> Timebase {
        Timebase(self.0)
    }
}

#[derive(Default)]
enum Preset {
    Fps24,
    #[default]
    Fps30,
    Fps60,
}

impl From<Preset> for Framerate {
    fn from(preset: Preset) -> Self {
        match preset {
            Preset::Fps24 => Framerate(24),
            Preset::Fps30 => Framerate(30),
            Preset::Fps60 => Framerate(60),
        }
    }
}

pub struct Timebase(u8);
impl Into<Rational> for Timebase {
    fn into(self) -> Rational {
        Rational::new(1, self.0.into())
    }
}
