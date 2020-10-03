use lazy_static::lazy_static;

use super::TagHandler;
use super::StructuredPrinter;

use crate::common::get_tag_attr;
use crate::dummy::IdentityHandler;

use regex::Regex;
use markup5ever_rcdom::Handle;

lazy_static! {
    /// Pattern that detects iframes with Youtube embedded videos
    /// Examples:
    /// * `https://www.youtube.com/embed/zE-dmXZp3nU?wmode=opaque`
    /// * `https://www.youtube-nocookie.com/embed/5yo6exIypkY`
    /// * `https://www.youtube.com/embed/TXm6IXrbQuM`
    static ref YOUTUBE_PATTERN : Regex = Regex::new(r"www\.youtube(?:-nocookie)?\.com/embed/([-\w]+)").unwrap();

    /// Pattern that detects iframes with Instagram embedded photos
    /// Examples:
    /// * `https://www.instagram.com/p/B1BKr9Wo8YX/embed/`
    /// * `https://www.instagram.com/p/BpKjlo-B4uI/embed/`
    static ref INSTAGRAM_PATTERN: Regex = Regex::new(r"www\.instagram\.com/p/([-\w]+)/embed").unwrap();

    static ref YANDEX_MUSIC_TRACK_PATTERN: Regex = Regex::new(r"https://music.yandex.ru/iframe/#track/(\d+)/(\d+)").unwrap();
    static ref YANDEX_MUSIC_ALBUM_PATTERN: Regex = Regex::new(r"https://music.yandex.ru/iframe/#album/(\d+)").unwrap();
}

#[derive(Default)]
pub(super) struct IframeHandler {
}

impl TagHandler for IframeHandler {

    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        printer.insert_newline();
        printer.insert_newline();

        let src = get_tag_attr(tag, "src");
        //let width = get_tag_attr(tag, "width");
        //let height = get_tag_attr(tag, "height");

        if src == None {
            return;
        }

        let src = src.unwrap();

        if let Some(capture) = YOUTUBE_PATTERN.captures(&src) {
            let media_id = capture.get(1).map_or("", |m| m.as_str());
            printer.append_str(&format!("[![Embedded YouTube video](https://img.youtube.com/vi/{mid}/0.jpg)](https://www.youtube.com/watch?v={mid})", mid = media_id));
            return
        }

        if let Some(capture) = INSTAGRAM_PATTERN.captures(&src) {
            let media_id = capture.get(1).map_or("", |m| m.as_str());
            printer.append_str(&format!("[![Embedded Instagram post](https://www.instagram.com/p/{mid}/media/?size=m)](https://www.instagram.com/p/{mid}/embed/)", mid = media_id));
            return
        }

        // not found, use generic implementation
        let mut identity = IdentityHandler::default();
        identity.handle(tag, printer);
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        printer.insert_newline();
        printer.insert_newline();
    }

    fn skip_descendants(&self) -> bool {
        return true;
    }
}