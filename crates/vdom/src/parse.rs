use std::str;

use treexml::{Document, Element};

pub fn create_tree(html: &str) -> Option<Element> {
    let doc;
    let root;

    match Document::parse(html.as_bytes()) {
        Ok(val) => doc = val,
        Err(_) => return None,
    }

    match doc.root {
        Some(val) => root = val,
        None => return None,
    }

    Some(root)
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_HTML: &str = "<article><header class=\"post-header\"><h1>Title</h1></header><section class=\"post-body\">Body</section></article>";

    #[test]
    fn create_tree_test() {
        match create_tree(RAW_HTML) {
            Some(val) => assert_eq!(
                val.children.len(),
                2,
                "create_tree() failed to produce a correct vDOM"
            ),
            None => assert!(false, "create_tree() failed to produce a vDOM"),
        }
    }
}
