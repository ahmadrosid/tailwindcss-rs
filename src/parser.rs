extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

use html5ever::driver::ParseOpts;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use rcdom::{Handle, NodeData, RcDom};
use std::error::Error;
use std::path::Path;

struct Css {
    source: String,
}

impl Css {
    pub fn new() -> Self {
        Self {
            source: String::new(),
        }
    }

    pub fn push(&mut self, class: &str) {
        self.source.push_str(class);
        self.source.push_str(" ");
    }
}

fn collect_css(node: &Handle, css: &mut Css) {
    match node.data {
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            assert!(name.ns == ns!(html));
            for attr in attrs.borrow().iter() {
                assert!(attr.name.ns == ns!());
                if &*attr.name.local == "class" {
                    css.push(&*attr.value);
                }
            }
        }
        NodeData::ProcessingInstruction { .. } => unreachable!(),
        _ => {}
    }

    for child in node.children.borrow().iter() {
        collect_css(child, css);
    }
}

fn walk(handle: &Handle) -> String {
    let mut css = Css::new();

    collect_css(handle, &mut css);

    css.source
}

pub(crate) fn process(path: &Path) -> Result<String, Box<dyn Error>> {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .from_file(path)?;
    let result = walk(&dom.document);

    Ok(result)
}
