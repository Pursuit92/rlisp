extern crate regex;

use val::Val;
use val::Val::{Ident,Bool,Num,Nil};

use std::f64;
use std::str::FromStr;
use std::iter::Peekable;
use std::collections::HashMap;

use self::regex::Regex;

#[derive(Eq,Hash,PartialEq,PartialOrd,Debug,Clone)]
enum TokType {
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

struct Lexer(pub HashMap<TokType, Regex>);
pub struct Parser(pub Lexer);

#[derive(Debug)]
struct Token<'a>(TokType, &'a str);

struct LexIter<'a> {
    lex: &'a Lexer,
    input: &'a str,
    line: usize,
    col: usize,
    pos: usize,
}

impl<'a> LexIter<'a> {
    pub fn parse(self) -> ParseIter<'a> {
        ParseIter(self.peekable())
    }

    pub fn error(&self, msg: &str) {
        println!("{} at line: {}, col: {}", msg, self.line, self.col)
    }
}

pub struct ParseIter<'a>(Peekable<LexIter<'a>>);

impl<'a> Iterator for ParseIter<'a> {
    type Item = Val;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.peek() {
            Some(&Token(TokType::LPAREN, _)) => self.parse_list(),
            _ => match self.0.next() {
                Some(Token(tok_type, text)) => match tok_type {
                    TokType::IDENT => Some(Ident(text.to_string())),
                    TokType::STRING => Some(text[1..text.len()-1].into()),
                    TokType::BOOLEAN => match text {
                        "#t" | "#T" => Some(Bool(true)),
                        _ => Some((Bool(false))),
                    },
                    TokType::NUMBER => Some(Num(f64::from_str(text).unwrap())),
                    TokType::SQUOTE => match self.next() {
                        Some(v) => Some(Val::cons(Ident("quote".to_string()), Val::cons(v, Nil))),
                        None => None,
                    },
                    _ => self.next(),
                },
                None => None,
            }
        }
    }
}

impl<'a> ParseIter<'a> {
    fn parse_list(&mut self) -> Option<Val> {
        self.0.next(); // skip open paren
        let mut l = Some(Nil);

        match self.0.peek() {
            Some(&Token(TokType::RPAREN, _)) => {},
            Some(_) => { l = self.parse_pair() },
            None => return None,
        };
        self.0.next(); // skip close paren
        l
    }

    fn parse_pair(&mut self) -> Option<Val> {
        match self.next() {
            Some(head) => match self.0.peek() {
                Some(&Token(TokType::RPAREN, _)) => Some(Val::cons(head, Nil)),
                Some(&Token(TokType::DOT, _)) => match self.next() {
                    Some(tail) => Some(Val::cons(head, tail)),
                    None => return None,
                },
                Some(_) => match self.parse_pair() {
                    Some(tail) => Some(Val::cons(head, tail)),
                    None => None,
                },
                None => return None,
            },
            None => return None,
        }
    }
}

impl<'a> Iterator for LexIter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lex.match_str(&self.input[self.pos..]) {
            None => None,
            Some((TokType::EOF,_)) => None,
            Some((tok_type, end)) => match tok_type {
                TokType::EOF => None,
                TokType::WS => {
                    self.pos += end;
                    self.col += end;
                    self.next()
                },
                TokType::NL => {
                    self.pos += end;
                    self.col = 0;
                    self.line += end;
                    self.next()
                },
                _ => {
                    let ret = Some(Token(tok_type, &self.input[self.pos..self.pos+end]));
                    self.pos += end;
                    ret
                }
            },
        }
    }
}

impl Parser {
    pub fn new() -> Parser {
        Parser(Lexer::new())
    }
    pub fn parse<'a>(&'a self, input: &'a str) -> ParseIter {
        self.0.lex(input).parse()
    }
}

impl Lexer {
    pub fn new() -> Lexer {
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
        Lexer(m)
    }
    fn match_str<'a>(&self, s: &'a str) -> Option<(TokType, usize)> {
        let mut ret = None;
        for (re_type, re) in self.0.iter() {
            match re.find(&*s) {
                Some((0, end)) => {
                    let new_ret = ((*re_type).clone(), end);
                    match ret {
                        None => {
                            ret = Some(new_ret);
                        },
                        Some((_, curr)) => {
                            if curr < end {
                                ret = Some(new_ret);
                            }
                        }
                    }
                },
                _ => {},
            }
        }
        ret
    }
    pub fn lex<'a>(&'a self, s: &'a str) -> LexIter<'a> {
        LexIter{
            lex: self,
            input: s,
            line: 0,
            col: 0,
            pos: 0,
        }
    }
}
