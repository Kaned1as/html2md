use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::{Handle,NodeData};

#[derive(Default)]
pub(super) struct AnchorHandler {
    start_pos: usize,
    url: String
}

impl TagHandler for AnchorHandler {
    
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        self.start_pos = printer.position;

        // try to extract a hyperlink
        self.url = match tag.data {
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
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        // add braces around already present text, put an url afterwards
        printer.position = self.start_pos;
        printer.insert_str("[");
        printer.position = printer.data.len();
        printer.insert_str(&format!("]({})", self.url))
    }
}