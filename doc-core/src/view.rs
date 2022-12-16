#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use linked_hash_map::LinkedHashMap;

use crate::config::ValueWrapper;

pub struct View {
    path: String,
    type_id: Option<String>,
    child_view: HashMap<String, ValueWrapper>,
}

#[derive(Debug)]
pub(crate) struct NodeView {
    type_id: Option<String>,
    cfg: HashMap<String, Rc<RefCell<ValueWrapper>>>,
    parent: RefCell<Weak<NodeView>>,
    child: RefCell<LinkedHashMap<String, Rc<RefCell<NodeView>>>>,
}

#[derive(Debug)]
pub struct Document {
    root: Option<Rc<RefCell<NodeView>>>,
}

impl Document {
    pub fn new() -> Self {
        Document {
            root: None
        }
    }
}
