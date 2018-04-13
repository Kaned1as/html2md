use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::{Handle,NodeData};

#[derive(Default)]
pub(super) struct TableHandler {
    start_pos: usize,
    column_count: usize,
    row_count: usize
}

impl TagHandler for TableHandler {
    
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        // let's detect column count
        let table_children = tag.children.borrow();
        let header = table_children.iter().find(|child| tag_name(&child) == "thead");
        if let Some(header_tag) = header {
            let header_children = header_tag.children.borrow().len();
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        
    }
}

fn tag_name(tag: &Handle) -> String {
    return match tag.data {
        NodeData::Element { ref name, .. } => name.local.to_string(),
        _ => String::new()
    }
}

fn  find_child<'a>(tag: &'a Handle, name: &str) -> Option<&'a Handle> {
    let children = tag.children.borrow();
    let direct_child = children.iter().find(|child| tag_name(&child) == name);
    if direct_child.is_none() {
        for child in children.iter() {
            let result = find_child(&child, name);
            if result.is_some() {
                return result;
            }
        }
    } else {
        return direct_child;
    }

    return None;
}

#[derive(Default)]
pub(super) struct TableHeaderHandler;

impl TagHandler for TableHeaderHandler {
    
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        // do nothing, inner <th> tags should fill first row
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        // need to insert delimiters between header and body
    }
}


/// Handler for th/tr tags inside table header or body
#[derive(Default)]
pub(super) struct TableRowHandler;

impl TagHandler for TableRowHandler {
    
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        let parent_table = printer.parent_chain.iter().rev().find(|&tag| tag == "thead" || tag == "tbody");
        if parent_table == None {
            // should not happen
            return;
        }

        if parent_table.unwrap() == "thead" {
            
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        
    }
}