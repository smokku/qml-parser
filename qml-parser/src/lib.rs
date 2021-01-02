#[macro_use]
extern crate pest_derive;

use pest::{error, iterators::Pair, Parser};
use std::cell::Cell;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Parser)]
#[grammar = "qml.pest"]
struct QMLPest;

#[cfg(test)]
mod tests;

#[derive(Default)]
pub struct QMLParser {
    imports: HashMap<String, HashMap<String, Box<Cell<dyn Generator>>>>,
    in_scope: Vec<Box<Cell<dyn Generator>>>,
}

pub trait Generator: Drop {
    fn create(&mut self, name: &str) -> Option<Box<Cell<dyn Generator>>>;
    fn insert_string(&mut self, attribute: &str, value: String);
    fn insert_integer(&mut self, attribute: &str, value: i32);
    fn insert_float(&mut self, attribute: &str, value: f32);
    fn done(&mut self);
}

impl QMLParser {
    pub fn new() -> Self {
        QMLParser::default()
    }

    pub fn register_import<S: Into<String>>(
        &mut self,
        name: S,
        version: S,
        gen: Box<Cell<dyn Generator>>,
    ) {
        let import = self.imports.entry(name.into()).or_insert_with(HashMap::new);
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
        let body = pairs.next().unwrap();

        if let Some(mut generator) = self
            .in_scope
            .iter_mut()
            .find_map(|gen| gen.get_mut().create(type_name.as_str()))
        {
            for pair in body.into_inner() {
                match pair.as_rule() {
                    Rule::attribute_assignment => {
                        let mut pairs = pair.into_inner();
                        let attribute = pairs.next().unwrap();
                        let value = pairs.next().unwrap();
                        println!("{}, {:?}", attribute.as_str(), value);
                        match value.as_rule() {
                            Rule::integer => {
                                generator.get_mut().insert_integer(
                                    attribute.as_str(),
                                    value.as_str().parse().unwrap(),
                                );
                            }
                            Rule::float => {
                                generator.get_mut().insert_float(
                                    attribute.as_str(),
                                    value.as_str().parse().unwrap(),
                                );
                            }
                            Rule::string => {
                                generator
                                    .get_mut()
                                    .insert_string(attribute.as_str(), value.to_string());
                            }
                            Rule::method_call
                            | Rule::method_body
                            | Rule::list
                            | Rule::object
                            | Rule::boolean
                            | Rule::identifier => {
                                return Err(error::Error::new_from_pos(
                                    error::ErrorVariant::<Rule>::CustomError {
                                        message: "Not supported".to_string(),
                                    },
                                    value.as_span().start_pos(),
                                ))
                            }
                            _ => {
                                unreachable!(value.as_str())
                            }
                        }
                    }
                    Rule::property_definition
                    | Rule::signal_definition
                    | Rule::method_attribute => {
                        return Err(error::Error::new_from_pos(
                            error::ErrorVariant::<Rule>::CustomError {
                                message: "Not supported".to_string(),
                            },
                            pair.as_span().start_pos(),
                        ))
                    }
                    _ => {
                        unreachable!(pair.as_str())
                    }
                }
            }
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
