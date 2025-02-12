use lazy_static::lazy_static;

use super::TagHandler;

use crate::common::get_tag_attr;
use crate::StructuredParser;

use regex::Regex;
use markup5ever_rcdom::Handle;

lazy_static! {
    /// Pattern that detects iframes with Youtube embedded videos<br/>
    /// Examples:
    /// * `https://www.youtube.com/embed/zE-dmXZp3nU?wmode=opaque`
    /// * `https://www.youtube-nocookie.com/embed/5yo6exIypkY`
    /// * `https://www.youtube.com/embed/TXm6IXrbQuM`
    static ref YOUTUBE_PATTERN : Regex = Regex::new(r"www\.youtube(?:-nocookie)?\.com/embed/([-\w]+)").unwrap();

    /// Pattern that detects iframes with Instagram embedded photos<br/>
    /// Examples:
    /// * `https://www.instagram.com/p/B1BKr9Wo8YX/embed/`
    /// * `https://www.instagram.com/p/BpKjlo-B4uI/embed/`
    static ref INSTAGRAM_PATTERN: Regex = Regex::new(r"www\.instagram\.com/p/([-\w]+)/embed").unwrap();

    /// Patter that detects iframes with VKontakte embedded videos<br/>
    /// Examples:
    /// * `https://vk.com/video_ext.php?oid=-49423435&id=456245092&hash=e1611aefe899c4f8`
    /// * `https://vk.com/video_ext.php?oid=-76477496&id=456239454&hash=ebfdc2d386617b97`
    static ref VK_PATTERN: Regex = Regex::new(r"vk\.com/video_ext\.php\?oid=(-?\d+)&id=(\d+)&hash=(.*)").unwrap();

    static ref YANDEX_MUSIC_TRACK_PATTERN: Regex = Regex::new(r"https://music.yandex.ru/iframe/#track/(\d+)/(\d+)").unwrap();
    static ref YANDEX_MUSIC_ALBUM_PATTERN: Regex = Regex::new(r"https://music.yandex.ru/iframe/#album/(\d+)").unwrap();
}

#[derive(Default)]
pub struct IframeHandler;

impl TagHandler for IframeHandler {

    fn before_handle(&mut self, tag: &Handle, printer: &mut StructuredParser) {
        // let src = get_tag_attr(tag, "src");
        //
        // if src == None {
        //     return;
        // }
        //
        // let src = src.unwrap();
        //
        // if let Some(capture) = YOUTUBE_PATTERN.captures(&src) {
        //     let media_id = capture.get(1).map_or("", |m| m.as_str());
        //     printer.append_str(&format!("[![Embedded YouTube video](https://img.youtube.com/vi/{mid}/0.jpg)](https://www.youtube.com/watch?v={mid})", mid = media_id));
        //     return
        // }
        //
        // if let Some(capture) = INSTAGRAM_PATTERN.captures(&src) {
        //     let media_id = capture.get(1).map_or("", |m| m.as_str());
        //     printer.append_str(&format!("[![Embedded Instagram post](https://www.instagram.com/p/{mid}/media/?size=m)](https://www.instagram.com/p/{mid}/embed/)", mid = media_id));
        //     return
        // }
        //
        // if let Some(capture) = VK_PATTERN.captures(&src) {
        //     let owner_id = capture.get(1).map_or("", |m| m.as_str());
        //     let video_id = capture.get(2).map_or("", |m| m.as_str());
        //     let _hash = capture.get(3).map_or("", |m| m.as_str());
        //     printer.append_str(&format!("[![Embedded VK video](https://st.vk.com/images/icons/video_empty_2x.png)](https://vk.com/video{oid}_{vid})", oid = owner_id, vid = video_id));
        //     return
        // }
    }

    fn after_handle(&mut self, _printer: &mut StructuredParser) {
    }

    fn skip_descendants(&self) -> bool {
        return true;
    }
}
