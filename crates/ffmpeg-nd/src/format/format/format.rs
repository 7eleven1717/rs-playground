#[derive(Default)]
pub enum Format {
    WebM,
    #[default]
    Mp4,
}

impl Format {
    pub fn short_name(&self) -> &'static str {
        match self {
            Format::WebM => "webm",
            Format::Mp4 => "mp4",
        }
    }
    pub fn mime_type(&self) -> &'static str {
        match self {
            Format::WebM => "video/webm",
            Format::Mp4 => "video/mp4",
        }
    }
}
