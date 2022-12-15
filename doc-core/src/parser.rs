#[allow(dead_code)]
#[derive(Debug)]
enum ValueWrapper {
    Float(f64),
    Number(i64),
    Text(String),
    Bool(bool),
}

impl ValueWrapper {
    fn to_string(&self) -> String {
        match &self {
            ValueWrapper::Float(f) => f.to_string(),
            ValueWrapper::Number(n) => n.to_string(),
            ValueWrapper::Text(t) => t.to_string(),
            ValueWrapper::Bool(b) => b.to_string(),
        }
    }
}
