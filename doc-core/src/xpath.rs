use std::ops::Not;

use linked_hash_map::LinkedHashMap;

use crate::error::DocError::SoftError;
use crate::error::DocResult;
use crate::error::ErrorType::{Format, Index};
use crate::view::TNodeView;
use crate::xpath::ViewPath::{ById, ByIndex, ByTypedId, ByTypedIndex};

#[cfg(test)]
mod view_path_test {
    use crate::xpath::ViewPath;

    #[test]
    fn test() {
        println!("{:?}", ViewPath::from_xpath("/param[12]/[123]/"))
    }
}

#[derive(Debug)]
pub(crate) enum ViewPath {
    ByIndex(usize),
    ByTypedIndex(String, usize),
    ById(String),
    ByTypedId(String, String),
}

impl ViewPath {
    pub(crate) fn filter<'a>(&self, child: &'a LinkedHashMap<String, TNodeView>) -> Option<&'a TNodeView> {
        match self {
            ByIndex(index) => if child.len() < *index {
                child.values().skip(*index).next()
            } else {
                None
            },
            ByTypedIndex(type_id, index) => {
                child.values().filter(|e| e.borrow().type_id == *type_id).skip(*index).next()
            }
            ById(type_id) => child.values().filter(|e| e.borrow().type_id == *type_id).next(),
            ByTypedId(type_id, id) => child.get(id).filter(|e| e.borrow().type_id == *type_id)
        }
    }
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
            if x[0].is_empty() {
                Ok(ById(value))
            } else {
                Ok(ByTypedId(x[0].to_string(), value))
            }
        } else {
            if x[0].is_empty() {
                Ok(ByIndex(value.parse::<usize>()
                    .map_err(|e| SoftError(Format(e.to_string())))?))
            } else {
                Ok(ByTypedIndex(x[0].to_string(), value.parse::<usize>()
                    .map_err(|e| SoftError(Format(e.to_string())))?))
            }
        }
    }
    pub(crate) fn from_xpath(xpath: &str) -> DocResult<Vec<ViewPath>> {
        let mut result = vec![];
        for item in xpath.split("/").filter(|e| e.trim().is_empty().not()).collect::<Vec<&str>>() {
            result.push(ViewPath::new(item)?)
        }
        Ok(result)
    }
}
