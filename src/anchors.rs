use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::{Handle,NodeData};

#[derive(Default)]
pub(super) struct AnchorHandler;

impl TagHandler for AnchorHandler {
    
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        // try to extract a hyperlink
        let url = match tag.data {
             NodeData::Element { ref attrs, .. } => {
                let attrs = attrs.borrow();
                let href = attrs.iter().find(|attr| attr.name.local.to_string() == "href");
                match href {
                    Some(link) => link.value.to_string(),
                    None => String::new()
                }
             }
             _ => String::new()
        };

        // at this point we know it's anchor tag
        printer.data.insert_str(printer.position, format!("[]({})", url).as_ref());

        // inserted a link, now we have to update position to move it one point forward, after "[" sign
        printer.position += 1
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        printer.position = printer.data.len();
    }
}