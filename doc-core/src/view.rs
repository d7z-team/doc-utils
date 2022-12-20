#![allow(dead_code)]
#![allow(unused_variables)]


use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicU64, Ordering};

use linked_hash_map::LinkedHashMap;

use crate::config::ValueWrapper;
use crate::error::{DocResult, ErrorType};
use crate::error::DocError::SoftError;
use crate::xpath::ViewPath;

pub(crate) type TNodeView = Rc<RefCell<NodeView>>;

#[derive(Debug, Default)]
pub(crate) struct NodeView {
    pub(crate) type_id: String,
    pub(crate) cfg: HashMap<String, ValueWrapper>,
    pub(crate) parent: Weak<RefCell<NodeView>>,
    pub(crate) child: LinkedHashMap<String, TNodeView>,
}


static ID: AtomicU64 = AtomicU64::new(0);

fn new_id() -> String {
    let i = ID.fetch_add(1, Ordering::Release);
    format!("doc-id-{}", i)
}


impl NodeView {
    fn add_group(current: &TNodeView) -> TNodeView {
        NodeView::add(current, "group".to_string(), new_id(), HashMap::new())
    }

    fn add(current: &Rc<RefCell<NodeView>>, type_id: String, id: String, config: HashMap<String, ValueWrapper>) -> Rc<RefCell<NodeView>> {
        let child = NodeView {
            type_id,
            cfg: config,
            parent: Rc::downgrade(current),
            child: LinkedHashMap::new(),
        };
        let mut current_mut = current.borrow_mut();
        let result = Rc::new(RefCell::new(child));
        let mut id = id;
        if id.trim().is_empty() {
            id = new_id();
        }
        current_mut.child.insert(id, Rc::clone(&result));
        result
    }
}

#[derive(Debug)]
pub struct Document {
    root: Rc<RefCell<NodeView>>,
}


impl Document {
    pub fn new() -> Self {
        Document {
            root: Rc::new(RefCell::new(NodeView::default()))
        }
    }
    fn add(&mut self, path: &str, type_id: String, id: String, conf: HashMap<String, ValueWrapper>, over: bool) -> DocResult<()> {
        let mut path: Vec<ViewPath> = ViewPath::from_xpath(path)?;
        Self::add_internal(&self.root, &mut path, type_id, id, conf, over)
    }
    fn add_internal(current: &TNodeView,
                    path: &mut Vec<ViewPath>,
                    type_id: String,
                    id: String,
                    conf: HashMap<String, ValueWrapper>,
                    over: bool) -> DocResult<()> {
        if path.is_empty() {
            // 到达末尾，结束
            NodeView::add(current, type_id, id, conf);
            return Ok(());
        }
        let current_key = path.remove(0);
        let ref_mut = current.borrow_mut();
        let option = current_key.filter(&ref_mut.child).or_else(||
            if over {
                NodeView::add_group(current)
            } else {
                None
            });

        if option.is_none() && over {
            println!("无子队列{:?}", current_key);
        }
        let selected = option.ok_or(SoftError(ErrorType::NotMatch))?;
        Self::add_internal(selected, path, type_id, id, conf, over)
    }
}

#[cfg(test)]
mod test_document {
    use std::collections::HashMap;
    use std::rc::Weak;

    use crate::view::Document;

    #[test]
    fn test() {
        let mut document = Document::new();
        document.add("/group[0]", "group".to_string(), "".to_string(), HashMap::new(), false).unwrap();
        let x = document.root.borrow();
        let weak = &x.child.values().next().unwrap().borrow().parent;
        println!("{:?}", Weak::upgrade(weak).unwrap().borrow().type_id);
        println!("{:#?}", document);
    }
}
