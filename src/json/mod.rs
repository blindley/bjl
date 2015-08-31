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

pub fn parse_json_string(json_str: &str) -> Option<JSON_Object> {
    match tokenize_json_string(json_str) {
        Some(tokens) => json_object_from_tokens(&tokens),
        None => None,
    }
}

fn json_object_from_tokens(tokens: &[JSON_Token]) -> Option<JSON_Object> {
    // check for outer braces
    if tokens.len() < 2 { return None; }
    if let JSON_Token::LBrace = tokens[0] {} else {
        return None;
    }

    if let JSON_Token::RBrace = tokens[tokens.len() - 1] {} else {
        return None;
    }

    // strip outer braces
    let inner = chop(tokens, 1, 1);
    json_object_from_inner_tokens(&inner)
}

fn json_object_from_inner_tokens(mut tokens: &[JSON_Token]) -> Option<JSON_Object> {
    let mut object = JSON_Object::new();
    while tokens.len() > 0 {
        let (head, tail) = peel_key_value_pair(tokens);
        match head {
            Some(pair) => { object.insert(pair.0, pair.1); },
            None => { return None; }
        }
        tokens = tail;
        if tokens.len() > 0 {
            if let JSON_Token::Comma = tokens[0] {
                tokens = chop_head(tokens, 1);
            } else {
                return None;
            }
        }
    }
    Some(object)
}

fn peel_key_value_pair(tokens: &[JSON_Token]) -> (Option<KeyValuePair>, &[JSON_Token]) {
    if tokens.len() < 3 { return (None, tokens); }
    let key = 
        match &tokens[0] {
            &JSON_Token::String(ref key) => key.clone(),
            _ => { return (None, tokens); }
        };

    match &tokens[1] {
        &JSON_Token::Colon => (),
        _ => { return (None, tokens); }
    }

    let tail = chop_head(tokens, 2);

    let (value, tail) = peel_value(tail);
    match value {
        Some(value) => {
            let pair = KeyValuePair(key, value);
            return (Some(pair), tail);
        },
        None => { return (None, tail); }
    }
}

fn peel_value(mut tokens: &[JSON_Token]) -> (Option<JSON_Value>, &[JSON_Token]) {
    if tokens.len() == 0 { return (None, tokens); }
    match &tokens[0] {
        &JSON_Token::LBrace => peel_object(tokens),
        &JSON_Token::LBracket => peel_array(tokens),
        &JSON_Token::Number(value) => {
            let tokens = chop_head(tokens, 1);
            let value = JSON_Value::Number(value);
            (Some(value), tokens)
        },
        &JSON_Token::String(ref value) => {
            let tokens = chop_head(tokens, 1);
            let value = JSON_Value::String(value.clone());
            (Some(value), tokens)
        },
        &JSON_Token::Bool(value) => {
            let tokens = chop_head(tokens, 1);
            let value = JSON_Value::Bool(value);
            (Some(value), tokens)
        },
        &JSON_Token::Null => {
            let tokens = chop_head(tokens, 1);
            let value = JSON_Value::Null;
            (Some(value), tokens)
        },
        _ => (None, tokens),
    }
}

fn peel_object(mut tokens: &[JSON_Token]) -> (Option<JSON_Value>, &[JSON_Token]) {
    if tokens.len() < 2 { return (None, tokens); }
    if let JSON_Token::LBrace = tokens[0] {} else {
        return (None, tokens);
    }

    let mut object = JSON_Object::new();

    tokens = chop_head(tokens, 1);
    let mut first = true;
    while tokens.len() > 0 {
        if let JSON_Token::RBrace = tokens[0] {
            return (Some(JSON_Value::Object(object)), chop_head(tokens, 1));
        }

        if !first {
            if let JSON_Token::Comma = tokens[0] {} else {
                return (None, tokens);
            }
            tokens = chop_head(tokens, 1);
        }

        let x = peel_key_value_pair(tokens);
        if let Some(pair) = x.0 {
            object.insert(pair.0, pair.1);
        } else {
            return (None, tokens);
        }
        tokens = x.1;

        first = false;
    }

    (None, tokens)
}

fn peel_array(mut tokens: &[JSON_Token]) -> (Option<JSON_Value>, &[JSON_Token]) {
    if tokens.len() < 2 { return (None, tokens); }
    if let JSON_Token::LBracket = tokens[0] {} else {
        return (None, tokens);
    }

    let mut array = JSON_Array::new();

    tokens = chop_head(tokens, 1);
    let mut first = true;
    while tokens.len() > 0 {
        if let JSON_Token::RBracket = tokens[0] {
            return (Some(JSON_Value::Array(array)), chop_head(tokens, 1));
        }

        if !first {
            if let JSON_Token::Comma = tokens[0] {} else {
                return (None, tokens);
            }
            tokens = chop_head(tokens, 1);
        }

        let x = peel_value(tokens);
        if let Some(value) = x.0 {
            array.push(value);
        } else {
            return (None, tokens);
        }
        tokens = x.1;

        first = false;
    }

    (None, tokens)
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

fn chop_head<T>(slice: &[T], head: usize) -> &[T] {
    &slice[head..slice.len()]
}

fn chop_tail<T>(slice: &[T], tail: usize) -> &[T] {
    &slice[0..(slice.len() - tail)]
}

fn chop<T>(slice: &[T], head: usize, tail: usize) -> &[T] {
    chop_head(chop_tail(slice, tail), head)
}