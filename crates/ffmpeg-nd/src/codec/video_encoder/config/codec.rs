use crate::codec::id::Id;

#[derive(Default, Copy, Clone)]
pub enum Codec {
    #[default]
    AV1,
    H264,
}

impl Codec {
    pub fn change_to_av1(&mut self) {
        *self = Codec::AV1;
    }
    pub fn change_to_h264(&mut self) {
        *self = Codec::H264;
    }
}

impl Into<Id> for Codec {
    fn into(self) -> Id {
        match self {
            Codec::AV1 => Id::AV1,
            Codec::H264 => panic!("H264 codec not implemented yet"),
        }
    }
}
