#![allow(dead_code)]
#![allow(unused_variables)]


use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Not;
use std::rc::{Rc, Weak};

use linked_hash_map::LinkedHashMap;

use crate::config::Config;
use crate::error::{DocResult, ErrorType};
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

    fn get_child(&self, path: &PathSelectType) -> Vec<Rc<NodeViewRef>> {
        match path {
            PathSelectType::Id(id) => self.child.get(id).map(|e| vec![e]).unwrap_or(vec![]),
            PathSelectType::Type(type_id) =>
                self.child.values().filter(|e| e.borrow().type_id == *type_id).collect(),
            PathSelectType::Tag(tag) =>
                self.child.values().filter(|e| e.borrow().tags.contains(tag)).collect(),
            PathSelectType::Index(index) =>
                self.child.iter().skip(*index).next().map(|e| vec![e.1]).unwrap_or(vec![]),
            PathSelectType::TypeId(type_id, id) =>
                self.child.get(id).filter(|e| e.borrow().type_id == *type_id)
                    .map(|e| vec![e]).unwrap_or(vec![]),
            PathSelectType::TypeIndex(type_id, index) =>
                self.child.values().filter(|e| e.borrow().type_id == *type_id).skip(*index).collect(),
            PathSelectType::TypeTag(type_id, tag) =>
                self.child.values().filter(|e| e.borrow().type_id == *type_id)
                    .filter(|e| e.borrow().tags.contains(tag)).collect(),
        }.iter().map(|e| Rc::clone(e)).collect()
    }
    fn add_child(current: &NodeViewRef, view: PushView) -> DocResult<()> {
        let id = if let Some(id) = view.id {
            if id.is_empty() || id.contains(" ") {
                return ErrorType::format_error("ID不能包含空格和特殊字符".to_owned());
            } else {
                id
            }
        } else {
            new_id()
        };
        let child = Rc::new(RefCell::new(NodeView {
            type_id: view.type_id,
            tags: view.tags,
            config: view.config,
            parent: Weak::new(),
            child: Default::default(),
        }));
        current.borrow_mut().child.insert(id, child);
        Ok(())
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
    pub fn exists(&self, path: &DocumentPath) -> bool {
        Self::exists_ref(&self.root, path, 0)
    }
    fn exists_ref(refs: &Rc<NodeViewRef>, path: &DocumentPath, index: usize) -> bool {
        let current = path.get(index);
        if current.is_none() || path.len() == index + 1 {
            // point moved to end.
            return true;
        }
        let current = current.unwrap();
        let child = NodeView::get_child(&*refs.borrow(), current);
        if child.is_empty() {
            false
        } else {
            Self::exists_ref(child.get(0).unwrap(), path, index + 1)
        }
    }


    pub fn mk_group(&self, path: &DocumentPath) -> bool {
        Self::mk_group_ref(&self.root, &path, 0).unwrap()
    }
    fn mk_group_ref(refs: &Rc<NodeViewRef>, path: &DocumentPath, index: usize) -> DocResult<bool> {
        let current = path.get(index);
        if current.is_none() {
            return Ok(false);
        }
        let current = current.unwrap();
        let child = {
            let child = refs.borrow();
            child.get_child(current)
        };
        if child.is_empty() {
            let id = if let PathSelectType::Id(id) = current {
                Some(id.to_string())
            } else {
                None
            };
            NodeView::add_child(refs, PushView {
                type_id: "group".to_string(),
                id,
                tags: vec![],
                config: Default::default(),
            })?;
            Ok(false)
        } else {
            Self::mk_group_ref(&child[0], path, index + 1)
        }
    }
    fn create(&self, path: DocumentPath, view: PushView) -> DocResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test_document {
    use crate::view::Document;
    use crate::xpath::PathSelectType;

    #[test]
    fn test() {
        let document = Document::new();
        let path = PathSelectType::from_path("/group/group").unwrap();
        document.mk_group(&path);
        println!("{}", document.exists(&path));
        println!("{:#?}", document.root.borrow());
    }
}
