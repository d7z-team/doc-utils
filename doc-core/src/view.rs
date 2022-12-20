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

#[derive(Debug)]
pub(crate) struct NodeView {
    pub(crate) type_id: String,
    pub(crate) config: HashMap<String, ValueWrapper>,
    pub(crate) parent: Weak<RefCell<NodeView>>,
    pub(crate) child: LinkedHashMap<String, TNodeView>,
}

impl Default for NodeView {
    fn default() -> Self {
        NodeView {
            type_id: "group".to_string(),
            config: HashMap::new(),
            parent: Weak::new(),
            child: LinkedHashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Document {
    root: TNodeView,
}


impl Document {
    pub fn new() -> Self {
        Document {
            root: Rc::new(RefCell::new(NodeView::default()))
        }
    }
}

#[cfg(test)]
mod test_document {

    #[test]
    fn test() {

    }
}
