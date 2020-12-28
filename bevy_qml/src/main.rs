use qml_parser::{Generator, QMLParser};
use std::fs;

pub type BoxError = std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;

#[derive(Debug, Default)]
struct UiGenerator {
    created: bool,
}

impl Generator for UiGenerator {
    fn done(&mut self) {
        self.created = true;
        println!("Done");
    }
}

impl Drop for UiGenerator {
    fn drop(&mut self) {
        if !self.created {
            self.done();
        }
    }
}

fn main() -> std::result::Result<(), BoxError> {
    let file = fs::read_to_string("bevy_qml/res/basic.qml")?;

    let mut parser = QMLParser::new();
    parser.register_import(
        "BevyUi",
        "0.4",
        Box::new(|_name: &str| Some(Box::new(UiGenerator::default()))),
    );
    parser.process(file.as_str())?;

    Ok(())
}
