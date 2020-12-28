#[macro_use]
extern crate pest_derive;

use pest::{error, iterators::Pair, Parser};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Parser)]
#[grammar = "qml.pest"]
struct QMLPest;

#[cfg(test)]
mod tests;

pub type GeneratorFn = dyn Fn(&str) -> Option<Box<dyn Generator>>;

pub struct QMLParser {
    imports: HashMap<String, HashMap<String, Box<GeneratorFn>>>,
    in_scope: Vec<Box<GeneratorFn>>,
}

pub trait Generator: Drop + Debug {
    fn done(&mut self);
}

impl QMLParser {
    pub fn new() -> Self {
        QMLParser {
            imports: HashMap::new(),
            in_scope: Vec::new(),
        }
    }

    pub fn register_import<S: Into<String>>(&mut self, name: S, version: S, gen: Box<GeneratorFn>) {
        let import = self
            .imports
            .entry(name.into())
            .or_insert_with(|| HashMap::new());
        import.insert(version.into(), gen);
    }

    pub fn process(&mut self, data: &str) -> Result<(), error::Error<Rule>> {
        self.in_scope.clear();

        let mut parsed = QMLPest::parse(Rule::qml, data)?;

        for pair in parsed.next().unwrap().into_inner() {
            match pair.as_rule() {
                Rule::import => {
                    self.process_import(pair)?;
                }
                Rule::object => {
                    self.process_object(pair)?;
                }
                Rule::pragma | Rule::EOI => {}
                _ => {
                    unreachable!(pair.as_str())
                }
            }
        }

        Ok(())
    }

    fn process_import(&mut self, pair: Pair<Rule>) -> Result<(), error::Error<Rule>> {
        let mut pairs = pair.into_inner();
        let name = pairs.next().unwrap();
        let version = pairs.next().unwrap();

        if let Some(import) = self.imports.get_mut(name.as_str()) {
            if let Some(import) = import.remove(version.as_str()) {
                self.in_scope.push(import);
                Ok(())
            } else {
                Err(error::Error::new_from_span(
                    error::ErrorVariant::<Rule>::CustomError {
                        message: "version not registered".to_string(),
                    },
                    version.as_span(),
                ))
            }
        } else {
            Err(error::Error::new_from_span(
                error::ErrorVariant::<Rule>::CustomError {
                    message: "import not registered".to_string(),
                },
                name.as_span(),
            ))
        }
    }

    fn process_object(&mut self, pair: Pair<Rule>) -> Result<(), error::Error<Rule>> {
        let mut pairs = pair.into_inner();
        let type_name = pairs.next().unwrap();
        let _body = pairs.next().unwrap();

        if let Some(generator) = self.in_scope.iter().find_map(|gen| gen(type_name.as_str())) {
            println!("{:?}", generator);
            Ok(())
        } else {
            Err(error::Error::new_from_span(
                error::ErrorVariant::<Rule>::CustomError {
                    message: "Object type not in scope".to_string(),
                },
                type_name.as_span(),
            ))
        }
    }
}
