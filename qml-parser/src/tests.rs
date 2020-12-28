use super::*;
use pest::{error, Parser};

#[test]
fn identifier() -> Result<(), error::Error<Rule>> {
    let mut pair = QMLPest::parse(Rule::identifier, "Object")?;
    assert_eq!(pair.next().unwrap().as_str(), "Object");
    Ok(())
}

#[test]
fn object() -> Result<(), error::Error<Rule>> {
    let mut pair = QMLPest::parse(Rule::object, "Object {}")?;

    let mut pairs = pair.next().unwrap().into_inner();
    let pair = pairs.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::type_name);
    let pair = pairs.next().unwrap();
    assert_eq!(pair.as_rule(), Rule::body);

    Ok(())
}

#[test]
fn body() -> Result<(), error::Error<Rule>> {
    let mut pair = QMLPest::parse(Rule::body, "{}")?;
    assert_eq!(pair.next().unwrap().as_str(), "{}");
    let mut pair = QMLPest::parse(Rule::body, "{property: 7}")?;
    assert_eq!(pair.next().unwrap().as_str(), "{property: 7}");
    let mut pair = QMLPest::parse(Rule::body, "{property: 0.7}")?;
    assert_eq!(pair.next().unwrap().as_str(), "{property: 0.7}");
    let mut pair = QMLPest::parse(Rule::body, "{property: \"aaa\"}")?;
    assert_eq!(pair.next().unwrap().as_str(), "{property: \"aaa\"}");
    let mut pair = QMLPest::parse(Rule::body, "{\tproperty:    7;}")?;
    assert_eq!(pair.next().unwrap().as_str(), "{\tproperty:    7;}");
    let mut pair = QMLPest::parse(Rule::body, "{property1:7;property2:\"abc\"}")?;
    assert_eq!(
        pair.next().unwrap().as_str(),
        "{property1:7;property2:\"abc\"}"
    );
    let mut pair = QMLPest::parse(Rule::body, "{\n  property1: 7\n  property2: 8\n}")?;
    assert_eq!(
        pair.next().unwrap().as_str(),
        "{\n  property1: 7\n  property2: 8\n}"
    );
    Ok(())
}

#[test]
fn number() -> Result<(), error::Error<Rule>> {
    let mut pair = QMLPest::parse(Rule::integer, "0")?;
    assert_eq!(pair.next().unwrap().as_str(), "0");
    let mut pair = QMLPest::parse(Rule::integer, "1")?;
    assert_eq!(pair.next().unwrap().as_str(), "1");
    let mut pair = QMLPest::parse(Rule::integer, "1234567890")?;
    assert_eq!(pair.next().unwrap().as_str(), "1234567890");
    let mut pair = QMLPest::parse(Rule::integer, "-1")?;
    assert_eq!(pair.next().unwrap().as_str(), "-1");
    let mut pair = QMLPest::parse(Rule::integer, "-0")?;
    assert_eq!(pair.next().unwrap().as_str(), "-0");

    let mut pair = QMLPest::parse(Rule::float, "0.0")?;
    assert_eq!(pair.next().unwrap().as_str(), "0.0");
    let mut pair = QMLPest::parse(Rule::float, "0.1")?;
    assert_eq!(pair.next().unwrap().as_str(), "0.1");
    let mut pair = QMLPest::parse(Rule::float, "1.0")?;
    assert_eq!(pair.next().unwrap().as_str(), "1.0");
    let mut pair = QMLPest::parse(Rule::float, "1.1")?;
    assert_eq!(pair.next().unwrap().as_str(), "1.1");
    let mut pair = QMLPest::parse(Rule::float, "1.234567890")?;
    assert_eq!(pair.next().unwrap().as_str(), "1.234567890");
    let mut pair = QMLPest::parse(Rule::float, "-1.0")?;
    assert_eq!(pair.next().unwrap().as_str(), "-1.0");
    let mut pair = QMLPest::parse(Rule::float, "-0.1")?;
    assert_eq!(pair.next().unwrap().as_str(), "-0.1");
    let mut pair = QMLPest::parse(Rule::float, "-0.0")?;
    assert_eq!(pair.next().unwrap().as_str(), "-0.0");
    let mut pair = QMLPest::parse(Rule::float, "10e100")?;
    assert_eq!(pair.next().unwrap().as_str(), "10e100");
    let mut pair = QMLPest::parse(Rule::float, "10e-100")?;
    assert_eq!(pair.next().unwrap().as_str(), "10e-100");
    let mut pair = QMLPest::parse(Rule::float, "-10e100")?;
    assert_eq!(pair.next().unwrap().as_str(), "-10e100");
    let mut pair = QMLPest::parse(Rule::float, "-10e-100")?;
    assert_eq!(pair.next().unwrap().as_str(), "-10e-100");

    Ok(())
}

#[test]
fn string() -> Result<(), error::Error<Rule>> {
    let mut pair = QMLPest::parse(Rule::string, "\"abcdef\"")?;
    assert_eq!(pair.next().unwrap().as_str(), "\"abcdef\"");
    let mut pair = QMLPest::parse(Rule::string, "\"ABCDEF\"")?;
    assert_eq!(pair.next().unwrap().as_str(), "\"ABCDEF\"");
    let mut pair = QMLPest::parse(Rule::string, "\"abc def\"")?;
    assert_eq!(pair.next().unwrap().as_str(), "\"abc def\"");
    let mut pair = QMLPest::parse(Rule::string, "\"dot...\"")?;
    assert_eq!(pair.next().unwrap().as_str(), "\"dot...\"");
    let mut pair = QMLPest::parse(Rule::string, "\"exclamation!\"")?;
    assert_eq!(pair.next().unwrap().as_str(), "\"exclamation!\"");
    Ok(())
}

#[test]
fn value() -> Result<(), error::Error<Rule>> {
    let mut pair = QMLPest::parse(Rule::value, "0")?;
    assert_eq!(pair.next().unwrap().as_str(), "0");
    let mut pair = QMLPest::parse(Rule::value, "1.1")?;
    assert_eq!(pair.next().unwrap().as_str(), "1.1");
    let mut pair = QMLPest::parse(Rule::value, "\"abcdef\"")?;
    assert_eq!(pair.next().unwrap().as_str(), "\"abcdef\"");
    Ok(())
}

#[test]
fn attribute_assignment() -> Result<(), error::Error<Rule>> {
    let mut pair = QMLPest::parse(Rule::attribute_assignment, "0")?;
    assert_eq!(pair.next().unwrap().as_str(), "0");
    let mut pair = QMLPest::parse(Rule::attribute_assignment, "1.1")?;
    assert_eq!(pair.next().unwrap().as_str(), "1.1");
    let mut pair = QMLPest::parse(Rule::attribute_assignment, "\"abcdef\"")?;
    assert_eq!(pair.next().unwrap().as_str(), "\"abcdef\"");
    let mut pair = QMLPest::parse(Rule::attribute_assignment, "property: 0")?;
    assert_eq!(pair.next().unwrap().as_str(), "property: 0");
    let mut pair = QMLPest::parse(Rule::attribute_assignment, "property: 1.1")?;
    assert_eq!(pair.next().unwrap().as_str(), "property: 1.1");
    let mut pair = QMLPest::parse(Rule::attribute_assignment, "property: \"abcdef\"")?;
    assert_eq!(pair.next().unwrap().as_str(), "property: \"abcdef\"");
    let mut pair = QMLPest::parse(Rule::attribute_assignment, "property: 0;")?;
    assert_eq!(pair.next().unwrap().as_str(), "property: 0;");
    let mut pair = QMLPest::parse(Rule::attribute_assignment, "property: 1.1;")?;
    assert_eq!(pair.next().unwrap().as_str(), "property: 1.1;");
    let mut pair = QMLPest::parse(Rule::attribute_assignment, "property: \"abcdef\";")?;
    assert_eq!(pair.next().unwrap().as_str(), "property: \"abcdef\";");
    Ok(())
}
