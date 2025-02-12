use crate::StructuredParser;

use super::TagHandler;

use std::{collections::HashMap, cmp};

use markup5ever_rcdom::{Handle,NodeData};

#[derive(Default)]
pub struct TableHandler;

impl TagHandler for TableHandler {

    fn before_handle(&mut self, tag: &Handle, printer: &mut StructuredParser) {
    }

    fn after_handle(&mut self, _printer: &mut StructuredParser) {

    }

    fn skip_descendants(&self) -> bool {
        return true;
    }
}


/// Extracts tag name from passed tag
/// Returns empty string if it's not an html element
fn tag_name(tag: &Handle) -> String {
    return match tag.data {
        NodeData::Element { ref name, .. } => name.local.to_string(),
        _ => String::new()
    }
}

/// Find descendants of this tag with tag name `name`
/// This includes both direct children and descendants
fn  find_children(tag: &Handle, name: &str) -> Vec<Handle> {
    let mut result: Vec<Handle> = vec![];
    let children = tag.children.borrow();
    for child in children.iter() {
        if tag_name(&child) == name {
            result.push(child.clone());
        }

        let mut descendants = find_children(&child, name);
        result.append(&mut descendants);
    }

    return result;
}

/// Collect direct children that satisfy the predicate
/// This doesn't include descendants
fn collect_children<P>(tag: &Handle, predicate: P) -> Vec<Handle>
where P: Fn(&Handle) -> bool {
    let mut result: Vec<Handle> = vec![];
    let children = tag.children.borrow();
    for child in children.iter() {
        let candidate = child.clone();
        if predicate(&candidate) {
            result.push(candidate);
        }
    }

    return result;
}

