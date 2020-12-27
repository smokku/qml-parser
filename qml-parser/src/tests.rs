use super::*;
use pest::{error, Parser};

#[test]
fn identifier() -> Result<(), error::Error<Rule>> {
    let mut pair = QMLPest::parse(Rule::identifier, "Object")?;
    let pair = pair.next().unwrap();
    assert_eq!(pair.as_str(), "Object");
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
    QMLPest::parse(Rule::body, "{}")?;
    QMLPest::parse(Rule::body, "{property: 7}")?;
    QMLPest::parse(Rule::body, "{\tproperty:    7;}")?;
    QMLPest::parse(Rule::body, "{property1:7;property2:\"abc\"}")?;
    QMLPest::parse(Rule::body, "{\n  property1: 7\n  property2: 8\n}")?;
    Ok(())
}
