//#![feature(alloc_system)]

extern crate html5ever;
#[macro_use]
extern crate lazy_static;

use html5ever::parse_document;
use html5ever::rcdom::{RcDom, Handle, NodeData};
use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::tree_builder::QuirksMode;
use html5ever::tokenizer::TokenizerOpts;
use html5ever::{QualName, Attribute};

struct State {

}

lazy_static! {
    static ref STATE: State = State {
        
    };
}

pub fn parse(html: &str) -> String {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            exact_errors: false,
            scripting_enabled: false,
            iframe_srcdoc: false,
            drop_doctype: true,
            ignore_missing_rules: true,
            quirks_mode: QuirksMode::NoQuirks
        },
        tokenizer: TokenizerOpts {
            exact_errors: false,
            discard_bom: true,
            profile: false,
            initial_state: None,
            last_start_tag_name: None
        }
    };
    let dom = parse_document(RcDom::default(), opts).from_utf8().read_from(&mut html.as_bytes()).unwrap();
    let mut result = String::new();
    walk(dom.document, &mut result);
    println!("{}", result);
    return result;
}

fn walk(input: Handle, result: &mut String) {
    match input.data {
        NodeData::Document => {},
        NodeData::Doctype { .. } => {},
        NodeData::Text { ref contents }
            => println!("#text: {}", &contents.borrow()),

        NodeData::Comment { ref contents }
            => println!("<!-- {} -->", contents),

        NodeData::Element { ref name, ref attrs, .. } => {
            match name.local.to_string().as_ref() {
                "html" | "head" | "body" => println!("skipping starting tags..."),
                "p" => result.push_str("\n\n"),
                "a" => handle_anchor(result, name, &attrs.borrow()), // should mark that we're inside the anchor
                _ => {}
            }
            print!("element {}", name.local);
            for attr in attrs.borrow().iter() {
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!()
        }

        NodeData::ProcessingInstruction { .. } => unreachable!()
    }

    for child in input.children.borrow().iter() {
        walk(child.clone(), result);
    }
}

fn handle_anchor(result: &mut String, name: &QualName, attrs: &Vec<Attribute>) {
    let url = attrs.into_iter().find(|attr| attr.name.local.to_string() == "href");
    if let Some(link) = url {
        result.push_str("[")
        STATE.
    }
    
}

#[cfg(test)]
mod tests {
    use parse;

    #[test]
    fn test() {
        parse("<p>aaaaa</p>");
    }
}