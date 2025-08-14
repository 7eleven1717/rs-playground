use crate::codec::Name;

#[derive(Default, Copy, Clone)]
pub enum Codec {
    #[default]
    Av1,
    H264,
}

impl Codec {
    pub fn change_to_av1(&mut self) {
        *self = Codec::Av1;
    }
    pub fn change_to_h264(&mut self) {
        *self = Codec::H264;
    }
}

impl From<Codec> for Name {
    fn from(codec: Codec) -> Name {
        match codec {
            Codec::Av1 => Name::LibAomAv1,
            Codec::H264 => Name::LibX264,
        }
    }
}
