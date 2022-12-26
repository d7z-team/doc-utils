#![allow(dead_code)]

use std::fmt::{Debug, Display, Formatter};

pub struct Source {
    pub lines: Vec<Vec<char>>,
    length: usize,
}

impl Debug for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for item in self.lines.iter()
            .map(|e| e.into_iter().collect::<String>())
            .collect::<Vec<String>>() {
            f.write_str(&item)?;
            f.write_str("\r\n")?;
        }
        Ok(())
    }
}

impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Source {
    pub fn from(data: &str) -> Self {
        Self::from_line_hook(data, |e| e.to_string())
    }
    pub fn from_line_hook(data: &str, hook: fn(&str) -> String) -> Self {
        let lines = data.lines().map(|e| hook(e).chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let length: usize = lines.iter().map(|e| e.len()).sum();
        Source {
            lines,
            length,
        }
    }
    pub fn from_prebuild_include(src: &str, path_hook: fn(&str) -> Option<String>) -> Self {
        let data = |line: &str| -> String {
            let option_start_index = line.find("[");
            if line.starts_with("include::") && line.ends_with("]") && option_start_index.is_some() {
                let option_start_index = option_start_index.unwrap();
                let path = line[9..option_start_index].to_string();
                let options = line[option_start_index..line.len() - 1].to_string();
                path_hook(&path).unwrap_or(format!(" {}", line))
            } else {
                line.to_string()
            }
        };
        let lines = src.lines().map(|e| data(e).chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let length: usize = lines.iter().map(|e| e.len()).sum();
        Source {
            lines,
            length,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::source::Source;

    #[test]
    fn test_new() {
        let source = Source::from(r#"line 1
测试行数 2
line 3"#);
        assert_eq!(source.length, 18);
        assert_eq!(source.lines[2], "line 3".chars().collect::<Vec<char>>())
    }
}
