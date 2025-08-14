pub struct Resolution {
    pub width: u16,
    pub height: u16,
}

impl Default for Resolution {
    fn default() -> Self {
        Preset::default().into()
    }
}

impl Resolution {
    pub fn pixel_count(&self) -> usize {
        (self.width as usize) * (self.height as usize)
    }
}

impl Resolution {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
    pub fn qvga() -> Self {
        Preset::QVGA.into()
    }
    pub fn vga() -> Self {
        Preset::VGA.into()
    }
    pub fn hd() -> Self {
        Preset::HD.into()
    }
    pub fn fhd() -> Self {
        Preset::FHD.into()
    }
    pub fn uhd4k() -> Self {
        Preset::UHD4K.into()
    }
    pub fn change_to_qvga(&mut self) {
        *self = Self::qvga();
    }
    pub fn change_to_vga(&mut self) {
        *self = Self::vga();
    }
    pub fn change_to_hd(&mut self) {
        *self = Self::hd();
    }
    pub fn change_to_fhd(&mut self) {
        *self = Self::fhd();
    }
    pub fn change_to_uhd4k(&mut self) {
        *self = Self::uhd4k();
    }
}

#[derive(Default)]
enum Preset {
    #[default]
    QVGA,
    VGA,
    HD,
    FHD,
    UHD4K,
}

impl From<Preset> for Resolution {
    fn from(preset: Preset) -> Self {
        match preset {
            Preset::QVGA => Resolution::new(320, 240),
            Preset::VGA => Resolution::new(640, 480),
            Preset::HD => Resolution::new(1280, 720),
            Preset::FHD => Resolution::new(1920, 1080),
            Preset::UHD4K => Resolution::new(3840, 2160),
        }
    }
}
