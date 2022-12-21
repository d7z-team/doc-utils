#![allow(dead_code)]
#![allow(unused_variables)]


use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Not;
use std::rc::{Rc, Weak};

use linked_hash_map::LinkedHashMap;

use crate::config::Config;
use crate::error::DocResult;
use crate::utils::new_id;
use crate::xpath::{DocumentPath, PathSelectType};

pub type NodeViewRef = RefCell<NodeView>;

#[derive(Debug)]
pub struct PushView {
    pub type_id: String,
    pub id: Option<String>,
    pub tags: Vec<String>,
    pub config: Config,
}

#[derive(Debug)]
pub struct NodeView {
    pub type_id: String,
    pub tags: Vec<String>,
    pub config: Config,
    pub parent: Weak<NodeViewRef>,
    pub child: LinkedHashMap<String, Rc<NodeViewRef>>,
}

impl Default for NodeView {
    fn default() -> Self {
        NodeView {
            type_id: "group".to_string(),
            tags: vec![String::from("group")],
            config: HashMap::new(),
            parent: Weak::new(),
            child: LinkedHashMap::new(),
        }
    }
}

impl NodeView {
    fn is_root(&self) -> bool {
        self.parent.upgrade().is_none()
    }
    fn get_child(&self, path: &PathSelectType) -> Vec<&Rc<NodeViewRef>> {
        match path {
            PathSelectType::Id(id) => self.child.get(id).map(|e| vec![e]).unwrap_or(vec![]),
            PathSelectType::Type(type_id) => self.child.values().filter(|e| e.borrow().type_id == *type_id).collect(),
            PathSelectType::Tag(tag) => self.child.values().filter(|e| e.borrow().tags.contains(tag)).collect(),
            PathSelectType::Index(index) => self.child.iter().skip(*index).next().map(|e| vec![e.1]).unwrap_or(vec![]),
            PathSelectType::TypeId(type_id, id) => self.child.get(id).filter(|e| e.borrow().type_id == *type_id)
                .map(|e| vec![e]).unwrap_or(vec![]),
            PathSelectType::TypeIndex(type_id, index) => self.child.values().filter(|e| e.borrow().type_id == *type_id).skip(*index).collect(),
            PathSelectType::TypeTag(type_id, tag) => self.child.values().filter(|e| e.borrow().type_id == *type_id)
                .filter(|e| e.borrow().tags.contains(tag)).collect(),
        }
    }
    fn add_child(current: &NodeViewRef, view: PushView) {
        let child = Rc::new(RefCell::new(NodeView {
            type_id: view.type_id,
            tags: view.tags,
            config: view.config,
            parent: Weak::new(),
            child: Default::default(),
        }));
        current.borrow_mut().child.insert(view.id.unwrap_or(new_id()), child);
    }
    fn has_child(&self, path: &PathSelectType) -> bool {
        self.get_child(path).is_empty().not()
    }
}

#[derive(Debug)]
pub struct Document {
    root: Rc<NodeViewRef>,
}


impl Document {
    pub fn new() -> Self {
        Document {
            root: Rc::new(RefCell::new(NodeView::default()))
        }
    }
    fn exists(&self, path: DocumentPath) -> bool {
        let mut child = Clone::clone(&path);
        Self::exists_ref(&self.root, &mut child)
    }
    fn exists_ref(refs: &Rc<NodeViewRef>, path: &mut DocumentPath) -> bool {
        if path.is_empty() {
            return false;
        }
        let select_type = path.remove(0);
        let current = refs.borrow();
        let child = current.get_child(&select_type);
        if child.is_empty() {
            return false;
        } else {
            Self::exists_ref(child[0], path)
        }
    }
    fn mk_group(&self, path: DocumentPath) -> bool {
        let mut child = Clone::clone(&path);
        Self::mk_group_ref(&self.root, &mut child)
    }
    fn mk_group_ref(refs: &Rc<NodeViewRef>, path: &mut DocumentPath) -> bool {
        if path.is_empty() {
            return false;
        }
        let rc1 = Rc::clone(refs);
        let select_type = path.remove(0);
        let current = rc1.borrow();
        let child = current.get_child(&select_type);
        if child.is_empty() {
            let id = if let PathSelectType::Id(id) = select_type {
                Some(id.to_string())
            } else {
                None
            };
            NodeView::add_child(refs, PushView {
                type_id: "group".to_string(),
                id,
                tags: vec![],
                config: Default::default(),
            });
            false
        } else {
            Self::mk_group_ref(child[0], path)
        }
    }
    fn create(&self, path: DocumentPath, view: PushView) -> DocResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test_document {
    use std::rc::Rc;

    use crate::view::Document;
    use crate::xpath::PathSelectType;

    #[test]
    fn test() {
        let document = Document::new();
        let result = PathSelectType::from_path("/item/data").unwrap();
        println!("{}", document.exists(Clone::clone(&result)));
        document.mk_group(result);
        println!("{:?}", document);
    }
}
