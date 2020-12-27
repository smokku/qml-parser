use qml_parser::QMLParser;
use std::fs;

pub type BoxError = std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;

fn main() -> std::result::Result<(), BoxError> {
    let file = fs::read_to_string("bevy_qml/res/basic.qml")?;

    let mut parser = QMLParser::new();
    parser.register_import("BevyUi", "0.4");
    parser.process(file.as_str())?;

    Ok(())
}
