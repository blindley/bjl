#![allow(dead_code)]

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum JSON_Token {
    LBrace, RBrace,
    LBracket, RBracket,
    Colon, Comma,
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

impl JSON_Token {
    pub fn is_lbrace(&self) -> bool {
        match *self { JSON_Token::LBrace => true, _ => false, }
    }

    pub fn is_rbrace(&self) -> bool {
        match *self { JSON_Token::RBrace => true, _ => false, }
    }

    pub fn is_lbracket(&self) -> bool {
        match *self { JSON_Token::LBracket => true, _ => false, }
    }

    pub fn is_rbracket(&self) -> bool {
        match *self { JSON_Token::RBracket => true, _ => false, }
    }

    pub fn is_colon(&self) -> bool {
        match *self { JSON_Token::Colon => true, _ => false, }
    }

    pub fn is_comma(&self) -> bool {
        match *self { JSON_Token::Comma => true, _ => false, }
    }

    pub fn is_number(&self) -> bool {
        match *self { JSON_Token::Number(..) => true, _ => false, }
    }

    pub fn is_string(&self) -> bool {
        match *self { JSON_Token::String(..) => true, _ => false, }
    }

    pub fn is_bool(&self) -> bool {
        match *self { JSON_Token::Bool(..) => true, _ => false, }
    }

    pub fn is_null(&self) -> bool {
        match *self { JSON_Token::Null => true, _ => false, }
    }

    pub fn unwrap_number(&self) -> f64 {
        match *self {
            JSON_Token::Number(value) => value,
            _ => { panic!(); }
        }
    }

    pub fn unwrap_string(&self) -> String {
        match *self {
            JSON_Token::String(ref value) => value.clone(),
            _ => { panic!(); }
        }
    }

    pub fn unwrap_bool(&self) -> bool {
        match *self {
            JSON_Token::Bool(value) => value,
            _ => { panic!(); }
        }
    }
}

pub fn tokenize_json_string(mut json: &str) -> Option<Vec<JSON_Token>> {
    let mut tokens: Vec<JSON_Token> = Vec::new();
    loop {
        let (token_opt, tail) = peel_json_token(json);
        match token_opt {
            Some(token) => tokens.push(token),
            None => break,
        }
        json = tail;
    }
    return Some(tokens);
}

fn str_contains(haystack: &str, needle: char) -> bool {
    for c in haystack.chars() {
        if c == needle { return true; }
    }
    return false;
}

fn str_contains_any_of(haystack: &str, needles: &str) -> bool {
    for c in haystack.chars() {
        for d in needles.chars() {
            if d == c { return true; }
        }
    }
    return false;
}

fn peel_one_char(s: &str) -> (&str, &str) {
    let head = &s[0..1];
    let tail = &s[1..s.len()];
    return (head, tail);
}

fn is_hexadecimal(c: char) -> bool {
    return str_contains("0123456789abcdefABCDEF", c);
}

fn peel_quoted_string(s: &str) -> (Option<&str>, &str) {
    let mut hex_count = 0;
    let mut escape = false;
    for c in (&s[1..s.len()]).char_indices() {
        if hex_count != 0 {
            if !is_hexadecimal(c.1) {
                break;
            }
            hex_count += 1;
            if hex_count == 5 {
                hex_count = 0;
            }
        } else if escape {
            if !str_contains("\"\\/bfnrtu", c.1) {
                break;
            } else if c.1 == 'u' {
                hex_count += 1;
            }
            escape = false;
        } else {
            if c.1 == '"' {
                let mid = c.0 + 2;
                let head = &s[0..mid];
                let tail = &s[mid..s.len()];
                return (Some(head), tail);
            } else if c.1 == '\\' {
                escape = true;
            }
        }
    }
    return (None, s);
}


fn peel_one_word(s: &str) -> (&str, &str) {
    let mut mid = s.len();
    let delimiters = " \t\n\r,:{}[]\"\'";
    for c in s.char_indices() {
        if str_contains(delimiters, c.1) {
            mid = c.0;
            break;
        }
    }
    return (&s[0..mid], &s[mid..s.len()]);
}

fn starts_with_any_of(haystack: &str, needles: &str) -> bool {
    for c in needles.chars() {
        if haystack.starts_with(c) { return true; }
    }
    return false;
}

fn trim_quotes(s: &str) -> &str {
    let mut start = 0;
    let mut end = s.len();
    if s.starts_with("\"") { start += 1; }
    if s.ends_with("\"") { end -= 1; }
    return &s[start..end];
}

fn peel_json_token(mut json: &str) -> (Option<JSON_Token>, &str) {
    json = json.trim();
    let mut token_opt: Option<JSON_Token> = None;
    let mut tail = json;

    if json.len() > 0 {
        if starts_with_any_of(json, "{}[]:,") {
            let (token_, tail_) = peel_one_char(json);
            let token_: JSON_Token =
                match token_ {
                    "{" => JSON_Token::LBrace,
                    "}" => JSON_Token::RBrace,
                    "[" => JSON_Token::LBracket,
                    "]" => JSON_Token::RBracket,
                    ":" => JSON_Token::Colon,
                    "," => JSON_Token::Comma,
                    _ => panic!("error in token processing"),
                };
            token_opt = Some(token_);
            tail = tail_;
        } else if json.starts_with('"') {
            let (token_, tail_) = peel_quoted_string(json);
            match token_ {
                Some(token_) => {
                    let token_ = (&token_[1..token_.len()-1]).to_owned();
                    token_opt = Some(JSON_Token::String(token_));
                    tail = tail_;
                },
                None => return (None, json),
            }
        } else {
            let (token_, tail_) = peel_one_word(json);
            let token_: JSON_Token = 
                match token_ {
                    "true" => JSON_Token::Bool(true),
                    "false" => JSON_Token::Bool(false),
                    "null" => JSON_Token::Null,
                    x @ _ => {
                        match x.parse::<f64>() {
                            Ok(x) => JSON_Token::Number(x),
                            Err(_) => return (None, json),
                        }
                    }
                };
            token_opt = Some(token_);
            tail = tail_;
        }
    }
    return (token_opt, tail);
}