#![allow(dead_code)]
#![allow(unused_variables)]


use std::collections::HashMap;

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
