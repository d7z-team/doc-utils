use crate::source::Source;
use crate::view::Document;

mod source;
mod parser;
mod view;
mod config;
mod error;
mod xpath;
mod utils;


#[cfg(test)]
mod test {
    use crate::source::Source;

    #[test]
    fn test() {
        let source = Source::from_prebuild_include(r#"
asdada
ada
include::sasasa[asasa]
adasda
adad
        "#.trim(), |e| -> Option<String>{
            println!("include {e}");
            Some(String::from(format!("include item by {}", e)))
        });
        println!("{}", source);
    }
}
