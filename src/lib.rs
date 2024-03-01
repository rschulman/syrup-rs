use preserves::value::{IOValue, Map, NestedValue, Value};
use winnow::ascii::digit1;
use winnow::combinator::{alt, dispatch, empty, fail, repeat_till};
use winnow::prelude::*;
use winnow::token::{any, one_of, take};
use winnow::PResult;

use std::collections::BTreeMap;

pub fn from_str(input: &mut &str) -> PResult<IOValue> {
    any_syrup(input)
}

fn any_syrup(input: &mut &str) -> PResult<IOValue> {
    alt((
        boolean, integer, bytestring, string, symbol, dictionary, //list, record, set,
    ))
    .parse_next(input)
}

fn boolean(input: &mut &str) -> PResult<IOValue> {
    dispatch! {any;
        't' => empty.value(IOValue::new(true)),
        'f' => empty.value(IOValue::new(false)),
        _ => fail::<_, IOValue, _>,
    }
    .parse_next(input)
}

fn integer(input: &mut &str) -> PResult<IOValue> {
    let (number, sign): (i64, char) = (digit1.parse_to(), one_of(['+', '-'])).parse_next(input)?;
    match sign {
        '+' => Ok(IOValue::new(number)),
        '-' => Ok(IOValue::new(number * -1)),
        _ => unreachable!(),
    }
}

fn bytestring(input: &mut &str) -> PResult<IOValue> {
    let (len, _): (usize, _) = (digit1.parse_to(), ":").parse_next(input)?;
    Ok(IOValue::new(take(len).parse_next(input)?.as_bytes()))
}

fn string(input: &mut &str) -> PResult<IOValue> {
    let (len, _): (usize, _) = (digit1.parse_to(), "\"").parse_next(input)?;
    Ok(IOValue::new(take(len).parse_next(input)?))
}

fn symbol(input: &mut &str) -> PResult<IOValue> {
    let (len, _): (usize, _) = (digit1.parse_to(), "\'").parse_next(input)?;
    Ok(IOValue::new(Value::symbol(take(len).parse_next(input)?)))
}

fn dictionary(input: &mut &str) -> PResult<IOValue> {
    let (_, (inner, _)): (_, (BTreeMap<IOValue, IOValue>, _)) =
        ("{", repeat_till(0.., (any_syrup, any_syrup), "}")).parse_next(input)?;
    let map_holder: Map<IOValue, IOValue> = Map::from_iter(inner);
    Ok(IOValue::new(map_holder))
}

#[cfg(test)]
mod tests {
    use preserves::value::{IOValue, NestedValue};

    #[test]
    fn test_boolean() {
        let mut input = "t";
        let response = crate::from_str(&mut input).expect("Failed to parse boolean test");

        assert_eq!(
            true,
            response
                .value_owned()
                .to_boolean()
                .expect("IOValue was not a boolean")
        );

        let mut second_input = "f";
        let second_response =
            crate::from_str(&mut second_input).expect("Failed to parse boolean test");

        assert_eq!(
            false,
            second_response
                .value_owned()
                .to_boolean()
                .expect("IOValue was not a boolean")
        );
    }

    #[test]
    fn test_positive_integer() {
        let mut input = "42+";
        let response = crate::from_str(&mut input).expect("Failed to parse integer test");

        assert_eq!(
            42,
            response
                .value_owned()
                .to_signedinteger()
                .expect("IOValue was not an integer")
                .try_into()
                .expect("Couldn't cast IOValue to i32")
        );
    }

    #[test]
    fn test_negative_integer() {
        let mut input = "756-";
        let response = crate::from_str(&mut input).expect("Failed to parse integer test");

        assert_eq!(
            -756,
            response
                .value_owned()
                .to_signedinteger()
                .expect("IOValue was not an integer")
                .try_into()
                .expect("Couldn't cast IOValue to i32")
        );
    }

    #[test]
    fn test_bytestring() {
        let mut input = "12:a bytestring";
        let response = crate::from_str(&mut input).expect("Failed to parse integer test");
        let expected: Vec<u8> = vec![
            0x61, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67,
        ];
        assert_eq!(
            expected,
            response
                .value_owned()
                .into_bytestring()
                .expect("IOValue was not a bytestring")
        );
    }

    #[test]
    fn test_string() {
        let mut input = "8\"a string";
        let response = crate::from_str(&mut input).expect("Failed to parse string test");
        assert_eq!(
            "a string".to_owned(),
            response
                .value_owned()
                .into_string()
                .expect("IOValue was not a string")
        );
    }

    #[test]
    fn test_symbol() {
        let mut input = "3'foo";
        let response = crate::from_str(&mut input).expect("Failed to parse symbol test");
        assert_eq!(
            "foo".to_owned(),
            response
                .value_owned()
                .into_symbol()
                .expect("IOValue was not a symbol")
        );
    }

    #[test]
    fn test_dictionary() {
        let mut input = "{4\"name3\"bob3\"age12+8\"favorite5\"pizza}";
        let response = crate::from_str(&mut input).expect("Failed to parse dictionary test");
        let mut expected = preserves::value::Map::new();
        expected.insert(IOValue::new("name"), IOValue::new("bob"));
        expected.insert(IOValue::new("age"), IOValue::new(12));
        expected.insert(IOValue::new("favorite"), IOValue::new("pizza"));
        assert_eq!(
            expected,
            response
                .value_owned()
                .into_dictionary()
                .expect("IOValue was not a dictionary")
        );
    }
}
