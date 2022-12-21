#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Not;

use crate::error::{DocError, DocResult};
use crate::error::ErrorType::Index;

#[derive(Debug, Clone)]
pub enum ValueWrapper {
    Float(f64),
    Number(i64),
    Text(String),
    Bool(bool),
    Array(Vec<String>),
    Map(HashMap<String, String>),
}

pub type Config = HashMap<String, ValueWrapper>;
