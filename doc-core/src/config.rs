#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Not;

use crate::error::{DocError, DocResult};
use crate::error::ErrorType::Index;

#[derive(Debug)]
pub enum ValueWrapper {
    Float(f64),
    Number(i64),
    Text(String),
    Bool(bool),
    Array(Vec<ValueWrapper>),
    Map(HashMap<String, ValueWrapper>),
}

impl ValueWrapper {
    fn to_string(&self) -> String {
        match &self {
            ValueWrapper::Float(f) => f.to_string(),
            ValueWrapper::Number(n) => n.to_string(),
            ValueWrapper::Text(t) => t.to_string(),
            ValueWrapper::Bool(b) => b.to_string(),
            ValueWrapper::Array(a) =>
                a.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(" ,"),
            ValueWrapper::Map(m) =>
                m.iter().map(|e| format!("{}={}", e.0, e.1.to_string())).collect::<Vec<String>>().join(" ;")
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub root: HashMap<String, ValueWrapper>,
}


impl Config {
    fn new() -> Self {
        Config {
            root: HashMap::new()
        }
    }
    fn push(&mut self, key: String, value: ValueWrapper) -> DocResult<()> {
        Self::push_internal(&mut self.root, key.split(".").collect::<Vec<&str>>(), value)
    }
    fn push_internal(node: &mut HashMap<String, ValueWrapper>, mut key: Vec<&str>, value: ValueWrapper) -> DocResult<()> {
        let current_key = if key.is_empty().not() {
            key.remove(0)
        } else {
            return Ok(());
        }.to_string();
        match node.entry(current_key) {
            Entry::Occupied(mut entry) => {
                if let ValueWrapper::Map(map) = entry.get_mut() {
                    Self::push_internal(map, key, value)?;
                } else {
                    return Err(DocError::SoftError(Index(format!("Cannot assign child node, node exists : {:?}", entry.get()))));
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(
                    if key.is_empty() {
                        value //到达末尾，插入结束数据
                    } else {
                        let mut child_node = HashMap::new();
                        Self::push_internal(&mut child_node, key, value)?;
                        ValueWrapper::Map(child_node)
                        // 未到达末尾，插入节点
                    }
                );
            }
        };
        return Ok(());
    }
}

#[cfg(test)]
mod test {
    use crate::config::{Config, ValueWrapper};

    #[test]
    fn test() {
        let mut config = Config::new();
        config.push("key.id.name".to_string(), ValueWrapper::Bool(false)).unwrap();
        config.push("key.id.id".to_string(), ValueWrapper::Text("data".to_string())).unwrap();
        println!("{:?}", config);
    }
}
