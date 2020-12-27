#[macro_use]
extern crate pest_derive;

pub use pest::Parser;

/// Type-erased errors.
pub type BoxError = std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;

#[derive(Parser)]
#[grammar = "qml.pest"]
pub struct QMLParser;

#[cfg(test)]
mod tests;
