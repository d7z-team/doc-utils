#![allow(dead_code)]
#![allow(unused_variables)]


use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicU64, Ordering};

use linked_hash_map::LinkedHashMap;

use crate::config::ValueWrapper;

type ViewContent = Rc<RefCell<View>>;

#[derive(Debug)]
pub struct View {
    pub name: Option<String>,
    pub config: HashMap<String, ValueWrapper>,
    pub parent: RefCell<Weak<View>>,
    pub child: RefCell<LinkedHashMap<String, ViewContent>>,
}

static ID: AtomicU64 = AtomicU64::new(0);

impl View {
    fn new_id() -> String {
        let i = ID.fetch_add(1, Ordering::Release);
        format!("doc-id-{}", i)
    }
    fn new() -> ViewContent {
        Rc::new(RefCell::new(View {
            name: None,
            config: HashMap::new(),
            parent: RefCell::default(),
            child: RefCell::new(LinkedHashMap::new()),
        }))
    }
    fn add_child(data: ViewContent, view: ViewContent) {
        let ref_mut = data.borrow_mut();
        let mut ref_mut1 = ref_mut.child.borrow_mut();
        let x = ref_mut1.deref_mut();
        x.insert(Self::new_id(), View::new());
    }
}

#[cfg(test)]
mod test {
    use crate::parser::View;

    #[test]
    fn test() {}
}
