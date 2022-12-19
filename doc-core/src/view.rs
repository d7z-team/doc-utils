#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Not;
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicU64, Ordering};

use linked_hash_map::LinkedHashMap;

use crate::config::ValueWrapper;
use crate::error::DocResult;
use crate::error::DocError::SoftError;
use crate::error::ErrorType::{Format, Index};
use crate::view::ViewPath::{ById, ByIndex};

type TNodeView = Rc<RefCell<NodeView>>;

#[derive(Debug)]
pub(crate) struct NodeView {
    type_id: String,
    cfg: HashMap<String, ValueWrapper>,
    parent: Weak<RefCell<NodeView>>,
    child: LinkedHashMap<String, TNodeView>,
}

static ID: AtomicU64 = AtomicU64::new(0);

fn new_id() -> String {
    let i = ID.fetch_add(1, Ordering::Release);
    format!("doc-id-{}", i)
}

impl Default for NodeView {
    fn default() -> Self {
        NodeView {
            type_id: "group".to_string(),

            cfg: HashMap::new(),
            parent: Weak::new(),
            child: LinkedHashMap::new(),
        }
    }
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
        let result: Vec<ViewPath> = ViewPath::from_xpath(path)?;
        let rc = Rc::clone(&self.root);
        let weak = Rc::downgrade(&rc);
        Ok(())
    }
    fn add_internal(current: &Rc<RefCell<NodeView>>,
                    path: &mut Vec<ViewPath>,
                    type_id: String,
                    id: String,
                    conf: HashMap<String, ValueWrapper>,
                    over: bool) -> DocResult<()> {
        if path.is_empty() {
            // point move  end , add view node
            NodeView::add(current, type_id, id, conf);
            return Ok(());
        }
        let current_key = path.remove(0);

        todo!()
    }
}

#[cfg(test)]
mod view_path_test {
    use crate::view::ViewPath;

    #[test]
    fn test() {
        println!("{:?}", ViewPath::from_xpath("/param[12]/[123]/"))
    }
}

#[derive(Debug)]
enum ViewPath {
    ByIndex(String, usize),
    ById(String, String),
}

impl ViewPath {
    fn new(item: &str) -> DocResult<ViewPath> {
        let types = if item.ends_with(")") {
            "("
        } else if item.ends_with("]") {
            "["
        } else {
            return Err(SoftError(Index("".to_string())));
        };
        let x = item.splitn(2, types).collect::<Vec<&str>>();
        if x.len() != 2 {
            return Err(SoftError(Index("".to_string())));
        }
        let value = x[1][0..x[1].len() - 1].to_string();
        if types == "(" {
            Ok(ById(x[0].to_string(), value))
        } else {
            Ok(ByIndex(x[0].to_string(), value.parse::<usize>()
                .map_err(|e| SoftError(Format(e.to_string())))?))
        }
    }
    fn from_xpath(xpath: &str) -> DocResult<Vec<ViewPath>> {
        let mut result = vec![];
        for item in xpath.split("/").filter(|e| e.trim().is_empty().not()).collect::<Vec<&str>>() {
            result.push(ViewPath::new(item)?)
        }
        Ok(result)
    }
}
