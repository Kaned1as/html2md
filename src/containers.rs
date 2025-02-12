use crate::StructuredParser;

use super::TagHandler;

use markup5ever_rcdom::Handle;
use markdown::mdast;

#[derive(Default)]
pub struct ContainerHandler;

impl TagHandler for ContainerHandler {

    fn before_handle(&mut self, _tag: &Handle, printer: &mut StructuredParser) {
        let container = mdast::Paragraph{children: vec![], position: None};
        printer.add_child(mdast::Node::Paragraph(container));
    }

    fn after_handle(&mut self, _printer: &mut StructuredParser) {
    }
}
