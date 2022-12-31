#![allow(dead_code)]
#![allow(unused_variables)]


use std::collections::HashMap;
use std::ops::Not;
use crate::config::ValueWrapper::{Array, Bool, Float, Map, Number};
use crate::error::{DocResult, ErrorType};

#[derive(Debug, Clone)]
pub enum ValueWrapper {
    Float(f64),
    Number(i64),
    Bool(bool),
    Array(Vec<String>),
    Map(HashMap<String, String>),
}

pub type Config = HashMap<String, ValueWrapper>;

impl ValueWrapper {
    pub(crate) fn from(src: &str) -> DocResult<Config> {
        let mut res = HashMap::new();
        let configs = src.split(";").collect::<Vec<&str>>();
        for config in configs {
            let params = config.splitn(2, "=").collect::<Vec<&str>>();
            let key = params[0].to_string();
            let value = if params.len() == 1 {
                Bool(true)
            } else {
                let value = params[1];
                value.parse::<f64>().map(|e| Float(e))
                    .or_else(|e| value.parse::<i64>().map(|e| Number(e)))
                    .or_else(|e| value.parse::<bool>().map(|e| Bool(e)))
                    .or_else(|e| Self::parse_map(value).map(|e| Map(e)))
                    .or_else(|e| Self::parse_vec(value).map(|e| Array(e)))
                    .unwrap_or(Array(vec![value.to_string()]))
            };
            match &value {
                Array(dist_arr) => {
                    if let Some(Array(arr)) = res.get_mut(&key) {
                        for dist in dist_arr {
                            arr.push(dist.to_string())
                        }
                    } else if res.contains_key(&key) {
                        return ErrorType::format_error(format!("Configuration {} already exists and cannot be converted to Array.", key));
                    } else {
                        res.insert(key, value);
                    }
                }
                Map(dist_map) => {
                    if let Some(Map(map)) = res.get_mut(&key) {
                        for dist in dist_map {
                            map.insert(dist.0.to_string(), dist.1.to_string());
                        }
                    } else if res.contains_key(&key) {
                        return ErrorType::format_error(format!("Configuration {} already exists and cannot be converted to Map.", key));
                    } else {
                        res.insert(key, value);
                    }
                }
                _ => {
                    res.insert(key, value);
                }
            }
        }

        return Ok(res);
    }
    fn parse_vec(value: &str) -> DocResult<Vec<String>> {
        if value.contains(",").not() {
            return ErrorType::format_error(format!("value {} not parse to vec.", value));
        }
        Ok(value.split(",").map(|e| e.to_string()).collect())
    }
    fn parse_map(value: &str) -> DocResult<HashMap<String, String>> {
        let mut res = HashMap::new();
        if value.contains(",").not() {
            return ErrorType::format_error(format!("value {} not parse to map.", value));
        }
        for item in value.split(",") {
            let data = item.splitn(2, ":").collect::<Vec<&str>>();
            if data.len() == 1 {
                return ErrorType::format_error(format!("value {}#{} not parse to map.", value, item));
            }
            res.insert(data[0].to_string(), data[1].to_string());
        }
        Ok(res)
    }
}
