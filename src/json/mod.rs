mod tokenize;
use self::tokenize::{JSON_Token, tokenize_json_string};

#[test]
fn test_main() {
    let json_str = include_str!("test_json.json");
    let tokens = tokenize_json_string(json_str).unwrap();
    let object = json_object_from_tokens(&tokens).unwrap();

    assert_eq!(object.len(), 2);
    assert!(object.contains_key("object"));
    assert!(object.contains_key("pengu"));
    assert_eq!(false, object.contains_key("objec"));

    if let Some(value) = object.get("object") {
        if let JSON_Value::Object(ref object) = *value {
            assert_eq!(object.len(), 5);
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}

#[derive(Debug, Clone)]
struct KeyValuePair(String, JSON_Value);

use std::collections::HashMap;
#[allow(non_camel_case_types)]
pub type JSON_Object = HashMap<String, JSON_Value>;

#[allow(non_camel_case_types)]
pub type JSON_Array = Vec<JSON_Value>;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum JSON_Value {
    String(String),
    Number(f64),
    Object(JSON_Object),
    Array(JSON_Array),
    Bool(bool),
    Null,
}

pub fn parse_json_string(json_str: &str) -> Option<JSON_Value> {
    if let Some(tokens) = tokenize_json_string(json_str) {
        if let Some((value, tail)) = peel_value(&tokens) {
            if tail.len() == 0 {
                match value {
                    JSON_Value::Object(..) | JSON_Value::Array(..)
                        => { return Some(value); },
                    _ => {},
                }
            }
        }
    }
    return None;
}

type PeelResult<'a, T> = Option<(T, &'a [JSON_Token])>;

fn chop_head<T>(slice: &[T], head: usize) -> &[T] {
    &slice[head..slice.len()]
}

fn chop_tail<T>(slice: &[T], tail: usize) -> &[T] {
    &slice[0..(slice.len() - tail)]
}

fn chop<T>(slice: &[T], head: usize, tail: usize) -> &[T] {
    chop_head(chop_tail(slice, tail), head)
}

fn json_object_from_tokens(tokens: &[JSON_Token]) -> Option<JSON_Object> {
    match peel_object(tokens) {
        Some((object, tail)) => {
            match tail.len() {
                0 => Some(object),
                _ => None,
            }
        },
        None => None,
    }
}

fn peel_key_value_pair(tokens: &[JSON_Token]) -> PeelResult<KeyValuePair> {
    if tokens.len() < 3 { return None; }
    if !tokens[0].is_string() { return None; }
    if !tokens[1].is_colon() { return None; }

    let key = tokens[0].unwrap_string();

    let tail = chop_head(tokens, 2);

    if let Some((value, tail)) = peel_value(tail) {
        let kvpair = KeyValuePair(key, value);
        return Some((kvpair, tail));
    } else {
        return None;
    }
}

fn peel_value(mut tokens: &[JSON_Token]) -> PeelResult<JSON_Value> {
    if tokens.len() == 0 { return None; }
    match &tokens[0] {
        &JSON_Token::LBrace => peel_object_as_value(tokens),
        &JSON_Token::LBracket => peel_array_as_value(tokens),
        &JSON_Token::Number(value) => {
            let tokens = chop_head(tokens, 1);
            let value = JSON_Value::Number(value);
            Some((value, tokens))
        },
        &JSON_Token::String(ref value) => {
            let tokens = chop_head(tokens, 1);
            let value = JSON_Value::String(value.clone());
            Some((value, tokens))
        },
        &JSON_Token::Bool(value) => {
            let tokens = chop_head(tokens, 1);
            let value = JSON_Value::Bool(value);
            Some((value, tokens))
        },
        &JSON_Token::Null => {
            let tokens = chop_head(tokens, 1);
            let value = JSON_Value::Null;
            Some((value, tokens))
        },
        _ => None,
    }
}

fn peel_object(mut tokens: &[JSON_Token]) -> PeelResult<JSON_Object> {
    if tokens.len() < 2 { return None; }
    if !tokens[0].is_lbrace() { return None; }

    let mut object = JSON_Object::new();

    tokens = chop_head(tokens, 1);
    let mut first = true;
    while tokens.len() > 0 {
        if tokens[0].is_rbrace() {
            let tokens = chop_head(tokens, 1);
            return Some((object, tokens));
        }

        if !first {
            if !tokens[0].is_comma() { return None; }
            tokens = chop_head(tokens, 1);
        }

        match peel_key_value_pair(tokens) {
            Some((kvpair, tail)) => {
                object.insert(kvpair.0, kvpair.1);
                tokens = tail;
            },
            None => { return None; }
        }
        first = false;
    }
    None
}

fn peel_object_as_value(tokens: &[JSON_Token]) -> PeelResult<JSON_Value> {
    match peel_object(tokens) {
        Some((object, tail)) => Some((JSON_Value::Object(object), tail)),
        None => None,
    }
}

fn peel_array(mut tokens: &[JSON_Token]) -> PeelResult<JSON_Array> {
    if tokens.len() < 2 { return None; }
    if !tokens[0].is_lbracket() {
        return None;
    }

    let mut array = JSON_Array::new();

    tokens = chop_head(tokens, 1);
    let mut first = true;
    while tokens.len() > 0 {
        if tokens[0].is_rbracket() {
            let tokens = chop_head(tokens, 1);
            return Some((array, tokens));
        }

        if !first {
            if !tokens[0].is_comma() { return None; }
            tokens = chop_head(tokens, 1);
        }

        match peel_value(tokens) {
            Some((value, tail)) => {
                array.push(value);
                tokens = tail;
            },
            None => { return None; }
        }
        first = false;
    }
    None
}

fn peel_array_as_value(tokens: &[JSON_Token]) -> PeelResult<JSON_Value> {
    match peel_array(tokens) {
        Some((array, tail)) => Some((JSON_Value::Array(array), tail)),
        None => None,
    }
}

