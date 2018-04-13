use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::{Handle,NodeData};

#[derive(Default)]
pub(super) struct ImgHandler;

impl TagHandler for ImgHandler {
    
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        // try to extract a hyperlink
        let (url, alt) = match tag.data {
             NodeData::Element { ref attrs, .. } => {
                let attrs = attrs.borrow();
                let src = attrs.iter().find(|attr| attr.name.local.to_string() == "src");
                let alt = attrs.iter().find(|attr| attr.name.local.to_string() == "alt");
                let url = match src {
                    Some(link) => link.value.to_string(),
                    None => String::new()
                };

                let text = match alt {
                    Some(text) => text.value.to_string(),
                    None => String::new()
                };

                (url, text)
             }
             _ => (String::new(), String::new())
        };
        

        // at this point we know it's anchor tag
        printer.data.insert_str(printer.position, format!("![{}]({})", alt, url).as_ref());

        // inserted a link, now we have to update position to move it one point forward, after "[" sign
        printer.position += 3
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        printer.position = printer.data.len();
    }
}