use lazy_static::lazy_static;

use super::TagHandler;
use super::StructuredPrinter;

use crate::common::get_tag_attr;

use regex::Regex;
use html5ever::rcdom::Handle;

lazy_static! {
    /// Pattern that detects iframes with Youtube embedded videos
    /// Examples: 
    /// * `https://www.youtube.com/embed/zE-dmXZp3nU?wmode=opaque`
    /// * `https://www.youtube.com/embed/5yo6exIypkY`
    /// * `https://www.youtube.com/embed/TXm6IXrbQuM`
    static ref YOUTUBE_PATTERN : Regex = Regex::new(r"www\.youtube\.com/embed/([-\w]+)").unwrap(); 
}

#[derive(Default)]
pub(super) struct IframeHandler {
}

/// We currently support only Youtube iframes
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

        if let Some(capture) = YOUTUBE_PATTERN.captures(&src.unwrap()) {
            let media_id = capture.get(1).unwrap();
            printer.append_str(&format!("[![Embedded video](https://img.youtube.com/vi/{mid}/0.jpg)](https://www.youtube.com/watch?v={mid})", mid = media_id.as_str()));
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        printer.insert_newline();
        printer.insert_newline();
    }
}