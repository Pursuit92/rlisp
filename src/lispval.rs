extern crate regex;

use std::collections::HashMap;
use list;
use std::iter::Peekable;

use self::regex::Regex;

#[derive(Debug)]
pub enum Val<'a> {
    Ident(&'a str),
    Bool(bool),
    Num(f64),
    String(&'a str),
    List(list::List<Box<Val<'a>>>)
}

#[derive(Eq,Hash,PartialEq,PartialOrd,Debug,Clone)]
pub enum TokType {
    EOF,
    WS,
    NL,

    LPAREN,
    RPAREN,
    COMMA,
    AT,
    DOT,
    BTICK,
    DQUOTE,
    SQUOTE,
    POUND,

    IDENT,
    BOOLEAN,
    NUMBER,
    CHARACTER,
    STRING,
}

struct Matcher(pub HashMap<TokType, Regex>);

#[derive(Debug)]
pub struct Token<'a>(TokType, &'a str);

pub struct Lexer<'a> {
    res: Matcher,
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer{ res: Matcher::new(), input: input }
    }
    pub fn parse(self) -> Parser<'a> {
        Parser(self.peekable())
    }
}

pub struct Parser<'a>(Peekable<Lexer<'a>>);

impl<'a> Iterator for Parser<'a> {
    type Item = Val<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.peek() {
            // Some(&Token(TokType::LPAREN, _)) => self.parse_list(),
            _ => match self.0.next() {
                Some(Token(tok_type, text)) => match tok_type {
                    //TokType::LPAREN => None,
                    TokType::IDENT => Some(Val::Ident(text)),
                    TokType::STRING => Some(Val::String(&text[1..text.len()-1])),
                    TokType::BOOLEAN => match text {
                        "#t" | "#T" => Some(Val::Bool(true)),
                        _ => Some((Val::Bool(false))),
                    },
                    _ => self.next(),
                },
                None => None,
            }
        }
    }
}

impl<'a> Parser<'a> {
    fn parse_list(&mut self) -> Option<Val<'a>> {
        self.next(); // skip open paren
        let l: list::List<Box<Val<'a>>> = list::List::new();
        loop {
            
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.res.match_str(self.input) {
            None => None,
            Some((TokType::EOF,_,_)) => None,
            Some((tok_type, text, rest)) => match tok_type {
                TokType::EOF => None,
                TokType::WS | TokType::NL => {
                    self.input = rest;
                    self.next()
                },
                _ => {
                    self.input = rest;
                    Some(Token(tok_type, text))
                }
            },
        }
    }
}

impl Matcher {
    pub fn new() -> Matcher {
        let mut m = HashMap::new();
        m.insert(TokType::EOF, Regex::new(r"^$").unwrap());
        m.insert(TokType::WS, Regex::new(r"[\t ]+").unwrap());
        m.insert(TokType::NL, Regex::new(r"\n+").unwrap());
        m.insert(TokType::LPAREN, Regex::new(r"\(").unwrap());
        m.insert(TokType::RPAREN, Regex::new(r"\)").unwrap());
        m.insert(TokType::COMMA, Regex::new(r",").unwrap());
        m.insert(TokType::AT, Regex::new(r"@").unwrap());
        m.insert(TokType::DOT, Regex::new(r"\.").unwrap());
        m.insert(TokType::BTICK, Regex::new(r"`").unwrap());
        m.insert(TokType::DQUOTE, Regex::new("\"").unwrap());
        m.insert(TokType::SQUOTE, Regex::new(r"'").unwrap());
        m.insert(TokType::POUND, Regex::new(r"#").unwrap());
        m.insert(TokType::IDENT, Regex::new(r"[a-zA-Z+!*%=<>_-][0-9a-zA-Z+!*%=<>_-]*").unwrap());
        m.insert(TokType::BOOLEAN, Regex::new(r"#[tTfF]").unwrap());
        m.insert(TokType::NUMBER, Regex::new(r"[0-9]*\.?[0-9]+").unwrap());
        m.insert(TokType::CHARACTER, Regex::new(r"#\\(newline|space|[a-zA-Z])").unwrap());
        m.insert(TokType::STRING, Regex::new(r#""(\\"|[^"])*""#).unwrap());
        Matcher(m)
    }
    pub fn match_str<'a>(&self, s: &'a str) -> Option<(TokType, &'a str, &'a str)> {
        let mut ret = None;
        for (re_type, re) in self.0.iter() {
            match re.find(&*s) {
                Some((0, end)) => {
                    ret = Some(((*re_type).clone(), &s[..end], &s[end..]));
                },
                _ => {},
            }
        }
        ret
    }
}
