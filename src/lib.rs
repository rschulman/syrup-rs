use preserves::value::{IOValue, Map, NestedValue, Set, Value};
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
        boolean, integer, bytestring, string, symbol, /* f32_float, f64_float, */ dictionary,
        list, record, set,
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

/* fn f32_float(input:&mut &str) -> PResult<IOValue> {
    let _= "F".parse_next(input)?;
    let number = be_f32.parse_next(input.as_bytes_mut())?;
    Ok(number)
} */

fn dictionary(input: &mut &str) -> PResult<IOValue> {
    let (_, (inner, _)): (_, (BTreeMap<IOValue, IOValue>, _)) =
        ("{", repeat_till(0.., (any_syrup, any_syrup), "}")).parse_next(input)?;
    let map_holder: Map<IOValue, IOValue> = Map::from_iter(inner);
    Ok(IOValue::new(map_holder))
}

fn list(input: &mut &str) -> PResult<IOValue> {
    let (_, (inner, _)): (_, (Vec<IOValue>, _)) =
        ("[", repeat_till(0.., any_syrup, "]")).parse_next(input)?;
    Ok(IOValue::new(inner))
}

fn record(input: &mut &str) -> PResult<IOValue> {
    let (_, (label, (inner, _))): (_, (IOValue, (Vec<IOValue>, _))) =
        ("<", (any_syrup, repeat_till(0.., any_syrup, ">"))).parse_next(input)?;

    let mut record = preserves::value::Value::record(label, inner.len() - 1);

    record.fields_vec_mut().extend_from_slice(&inner);
    Ok(record.finish().wrap())
}

fn set(input: &mut &str) -> PResult<IOValue> {
    let (_, (inner, _)): (_, (Set<IOValue>, _)) =
        ("#", repeat_till(0.., any_syrup, "$")).parse_next(input)?;
    Ok(IOValue::new(inner))
}