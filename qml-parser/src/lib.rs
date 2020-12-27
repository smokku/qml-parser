#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "qml.pest"]
struct QMLPest;

#[cfg(test)]
mod tests;

pub struct QMLParser {
    imports: HashMap<String, HashMap<String, ()>>,
    in_scope: Vec<()>,
}

impl QMLParser {
    pub fn new() -> Self {
        QMLParser {
            imports: HashMap::new(),
            in_scope: Vec::new(),
        }
    }

    pub fn register_import<S: Into<String>>(&mut self, name: S, version: S) {
        let import = self
            .imports
            .entry(name.into())
            .or_insert_with(|| HashMap::new());
        let version_import = import.entry(version.into()).or_insert_with(|| ());
        *version_import
    }

    pub fn process(&mut self, data: &str) -> Result<(), pest::error::Error<Rule>> {
        self.in_scope.clear();

        let mut parsed = QMLPest::parse(Rule::qml, data)?;

        for pair in parsed.next().unwrap().into_inner() {
            match pair.as_rule() {
                Rule::import | Rule::object => {
                    println!("{:?}", pair.as_rule());
                }
                Rule::pragma | Rule::EOI => {}
                _ => {
                    unreachable!(pair.as_str())
                }
            }
        }

        Ok(())
    }
}
