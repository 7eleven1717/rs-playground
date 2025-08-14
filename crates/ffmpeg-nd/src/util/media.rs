#![allow(non_upper_case_globals)]

use ffi::*;

#[derive(PartialEq)]
pub enum MediaType {
    Unknown,
    Video,
    Audio,
    Data,
    Subtitle,
    Attachment,
}

impl From<AVMediaType> for MediaType {
    fn from(value: AVMediaType) -> Self {
        match value {
            AVMediaType_AVMEDIA_TYPE_UNKNOWN => MediaType::Unknown,
            AVMediaType_AVMEDIA_TYPE_VIDEO => MediaType::Video,
            AVMediaType_AVMEDIA_TYPE_AUDIO => MediaType::Audio,
            AVMediaType_AVMEDIA_TYPE_DATA => MediaType::Data,
            AVMediaType_AVMEDIA_TYPE_SUBTITLE => MediaType::Subtitle,
            AVMediaType_AVMEDIA_TYPE_ATTACHMENT => MediaType::Attachment,
            AVMediaType_AVMEDIA_TYPE_NB => MediaType::Unknown,
            _ => panic!("Unsupported media type"),
        }
    }
}

impl Into<AVMediaType> for MediaType {
    fn into(self) -> AVMediaType {
        match self {
            MediaType::Unknown => AVMediaType_AVMEDIA_TYPE_UNKNOWN,
            MediaType::Video => AVMediaType_AVMEDIA_TYPE_VIDEO,
            MediaType::Audio => AVMediaType_AVMEDIA_TYPE_AUDIO,
            MediaType::Data => AVMediaType_AVMEDIA_TYPE_DATA,
            MediaType::Subtitle => AVMediaType_AVMEDIA_TYPE_SUBTITLE,
            MediaType::Attachment => AVMediaType_AVMEDIA_TYPE_ATTACHMENT,
        }
    }
}
