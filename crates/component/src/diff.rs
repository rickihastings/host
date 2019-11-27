use std::cmp::max;
use web_sys::{Element, Node, NodeList, NamedNodeMap, HtmlCollection};

type UnBoxedPatcher = dyn FnMut(&Element) -> &Element;
type Patcher = Box<dyn FnMut(&Element) -> &Element>;

struct Zipped {
    patcher: Patcher,
    children: Node,
}

fn zip(xs: Vec<UnBoxedPatcher>, ys: NodeList) -> Vec<Zipped> {
    let mut zipped: Vec<Zipped> = Vec::new();
    let ys_len = ys.length() as usize;

    for i in 0..max(xs.len(), ys_len) {
        if let Some(child) = ys.get(i as u32) {
            zipped.push(Zipped {
                patcher: Box::new(xs[i]),
                children: child
            })
        }
    }

    zipped
}

fn diff_attributes(old_attributes: &NamedNodeMap, new_attributes: &NamedNodeMap) -> Patcher {
    let mut patches: Vec<Patcher> = Vec::new();

    // Handle new attributes, or changing ones.
    for key in 0..new_attributes.length() {
        if let Some(attr) = new_attributes.get_with_index(key) {
            patches.push(Box::new(move |node: &Element| {
                node.set_attribute(&attr.name(), &attr.value());

                node
            }));
        }
    }

    // Remove old attributes
    for key in 0..old_attributes.length() {
        if let Some(attr) = new_attributes.get_with_index(key) {
            // Is the old attribute no longer in the new list?
            match new_attributes.get_with_name(&attr.name()) {
                None => {
                    patches.push(Box::new(move |node: &Element| {
                        node.remove_attribute(&attr.name());

                        node
                    }));
                },
                _ => {}
            }
        }
    }

    Box::new(move |node: &Element| {
        for patch in patches.iter_mut() {
            patch(node);
        }

        node
    })
}

fn diff_children(old_children: &HtmlCollection, new_children: &HtmlCollection) -> Patcher {
    let mut child_patches: Vec<UnBoxedPatcher> = Vec::new();

    for key in 0..old_children.length() {
        if let Some(old_child) = old_children.get_with_index(key) {
            // Need to diff old child with new child at the specified index
            if let Some(new_child) = new_children.get_with_index(key) {
                child_patches.push(move |node: &Element| {
                    diff_nodes(&old_child, &new_child);

                    node
                });
            }
        }
    }

    Box::new(move |node: &Element| {


        node
    })
}

pub fn diff_nodes(old_node: &Element, new_node: &Element) -> Patcher {
    // todo remove node

    // todo strings

    if old_node.tag_name() != new_node.tag_name() {
        // todo patch node
        Box::new(move |node: &Element| {
            node
        })
    } else {
        let mut patch_attributes = diff_attributes(&old_node.attributes(), &new_node.attributes());
        let mut patch_children = diff_children(&old_node.children(), &new_node.children());

        Box::new(move |node: &Element| {
            patch_attributes(node);
            patch_children(node);

            node
        })
    }
}
