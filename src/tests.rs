use super::*;
use pest::Parser;

#[test]
fn test_file() {
    let pair = QMLParser::parse(Rule::qml, "").unwrap().next().unwrap();

    assert_eq!(pair.as_rule(), Rule::qml);
}
