use crate::codec::Context as CodecContext;
use crate::format::{Context as FormatContext, Stream};
use crate::util::Error;

type MediaContext<'a> = (&'a mut FormatContext, &'a CodecContext);

impl<'a> TryFrom<MediaContext<'a>> for Stream {
    type Error = Error;

    fn try_from(media_context: MediaContext<'a>) -> Result<Self, Self::Error> {
        let stream = Stream::new(media_context.0);
        stream.parameters_from_context(media_context.1)?;
        Ok(stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use crate::codec::Name;
        use crate::format::Context;

        let codec_context = Name::default().default_video_context();
        let _media_context: MediaContext = (&mut Context::default(), &codec_context).into();
    }
}
