use super::TagHandler;
use super::StructuredPrinter;

use common::get_tag_attr;

use html5ever::rcdom::Handle;

use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

/// Handler for `<img>` tag. Depending on circumstances can produce both
/// inline HTML-formatted image and Markdown native one
#[derive(Default)]
pub(super) struct ImgHandler;

impl TagHandler for ImgHandler {
    
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        // try to extract attrs
        let src = get_tag_attr(tag, "src");
        let alt = get_tag_attr(tag, "alt");
        let title = get_tag_attr(tag, "title");
        let height = get_tag_attr(tag, "height");
        let width = get_tag_attr(tag, "width");
        let align = get_tag_attr(tag, "align");
        
        if height.is_some() || width.is_some() || align.is_some() {
            // need to handle it as inline html to preserve attributes we support
            printer.data.insert_str(printer.position, 
                &format!("<img{} />",
                    alt.map(|value| format!(" alt=\"{}\"", value)).unwrap_or_default() +
                    &src.map(|value| format!(" src=\"{}\"", value)).unwrap_or_default() +
                    &title.map(|value| format!(" title=\"{}\"", value)).unwrap_or_default() +
                    &height.map(|value| format!(" height=\"{}\"", value)).unwrap_or_default() +
                    &width.map(|value| format!(" width=\"{}\"", value)).unwrap_or_default() +
                    &align.map(|value| format!(" align=\"{}\"", value)).unwrap_or_default()));
        } else {
            // need to escape URL if it contains spaces
            // don't have any geometry-controlling attrs, post markdown natively
            printer.data.insert_str(printer.position, 
                &format!("![{}]({}{})", 
                    alt.unwrap_or_default(), 
                    utf8_percent_encode(&src.unwrap_or_default(), DEFAULT_ENCODE_SET),
                    title.map(|value| format!(" \"{}\"", value)).unwrap_or_default()));
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        // images can't have inner tags, it's ok
        printer.position = printer.data.len();
    }
}