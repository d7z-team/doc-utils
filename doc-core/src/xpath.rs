#![allow(dead_code)]

use std::num::ParseIntError;
use std::ops::Not;

use crate::error::{DocError, DocResult};
use crate::error::ErrorType::Format;
use crate::xpath::PathSelectType::{Id, Index, Tag, Type, TypeId, TypeIndex, TypeTag};

#[cfg(test)]
mod view_path_test {
    use crate::xpath::PathSelectType;
    use crate::xpath::PathSelectType::{Id, Index, Tag, Type, TypeId, TypeIndex, TypeTag};

    #[test]
    fn test() {
        assert_eq!(PathSelectType::from("title#h1").unwrap(), TypeId("title".to_string(), "h1".to_string()));
        assert_eq!(PathSelectType::from("title.h1").unwrap(), TypeTag("title".to_string(), "h1".to_string()));
        assert_eq!(PathSelectType::from("title:1").unwrap(), TypeIndex("title".to_string(), 1));
        assert_eq!(PathSelectType::from("#h1").unwrap(), Id("h1".to_string()));
        assert_eq!(PathSelectType::from(".h1").unwrap(), Tag("h1".to_string()));
        assert_eq!(PathSelectType::from(":1").unwrap(), Index(1));
        assert_eq!(PathSelectType::from("title").unwrap(), Type("title".to_string()));
    }
}

///
/// - type#id
/// - type.tag
/// - type:index
/// - #id
/// - .tag
/// - :index
/// - type
///
///
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PathSelectType {
    Type(String),
    Id(String),
    Tag(String),
    Index(usize),
    TypeId(String, String),
    TypeIndex(String, usize),
    TypeTag(String, String),
}

pub type DocumentPath = Vec<PathSelectType>;

impl PathSelectType {
    fn from(str: &str) -> DocResult<Self> {
        if str.matches(|current: char| current == '#' || current == ':' || current == '.' || current == ' ').count() > 1 {
            return Err(DocError::SoftError(Format(format!("'{}' 存在多个匹配字符或存在空格", str.to_string()))));
        }
        if str.contains("#") {
            if str.starts_with("#") {
                Ok(Id(str[1..].to_string()))
            } else {
                let pair = str.splitn(2, "#").collect::<Vec<&str>>();
                Ok(TypeId(pair[0].to_string(), pair[1].to_string()))
            }
        } else if str.contains(":") {
            if str.starts_with(":") {
                Ok(Index(str[1..].to_string().parse().map_err(|e: ParseIntError| {
                    DocError::SoftError(Format(e.to_string()))
                })?))
            } else {
                let pair = str.splitn(2, ":").collect::<Vec<&str>>();
                Ok(TypeIndex(pair[0].to_string(), pair[1].to_string().parse().map_err(|e: ParseIntError| {
                    DocError::SoftError(Format(e.to_string()))
                })?))
            }
        } else if str.contains(".") {
            if str.starts_with(".") {
                Ok(Tag(str[1..].to_string()))
            } else {
                let pair = str.splitn(2, ".").collect::<Vec<&str>>();
                Ok(TypeTag(pair[0].to_string(), pair[1].to_string()))
            }
        } else {
            Ok(Type(str.to_string()))
        }
    }
    pub fn from_path(path: &str) -> DocResult<Vec<Self>> {
        path.split("/").map(|e| e.trim()).filter(|e| e.is_empty().not()).map(|e| Self::from(e)).collect()
    }
}
