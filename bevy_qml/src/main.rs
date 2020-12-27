use qml_parser::*;
use std::fs;

fn main() -> std::result::Result<(), BoxError> {
    let unparsed_file = fs::read_to_string("bevy_qml/res/basic.qml")?;
    let mut file = QMLParser::parse(Rule::qml, &unparsed_file)?;
    let file = file.next().unwrap(); // get and unwrap the `file` rule; never fails

    println!("{:#?}", file);

    Ok(())
}
