#[derive(Debug)]
pub struct Source {
    lines: Vec<Vec<char>>,
    length: usize,
}


impl Source {
    pub fn new(data: &str) -> Self {
        let lines = data.lines().map(|e| e.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let length: usize = lines.iter().map(|e| e.len()).sum();
        Source {
            lines,
            length,
        }
    }
    pub fn to_string(&self) -> String {
        self.lines.iter()
            .map(|e| e.into_iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\r\n")
    }
}

#[cfg(test)]
mod test {
    use crate::source::Source;

    #[test]
    fn test_new() {
        let source = Source::new(r#"line 1
测试行数 2
line 3"#);
        assert_eq!(source.length, 18);
        assert_eq!(source.lines[2], "line 3".chars().collect::<Vec<char>>())
    }
}
