extern crate regex;

use std::collections::HashMap;
use list;
use std::iter::Peekable;
use std::fmt;
use std::f64;
use std::str::FromStr;

use self::regex::Regex;

#[derive(Debug)]
pub enum Val<'a> {
    Ident(&'a str),
    Bool(bool),
    Num(f64),
    String(&'a str),
    List(list::List<Box<Val<'a>>>)
}

impl<'a> fmt::Display for Val<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Val::Ident(istr) => write!(f, "{}", istr),
            &Val::Bool(b) => write!(f, "{}", b),
            &Val::Num(n) => write!(f, "{}", n),
            &Val::String(s) => write!(f, r#""{}""#, s),
            &Val::List(ref l) => write!(f, "{}", l),
        }
    }
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
    line: usize,
    col: usize,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer{ res: Matcher::new(), input: input, line: 0, col: 0, pos: 0 }
    }
    pub fn parse(self) -> Parser<'a> {
        Parser(self.peekable())
    }

    pub fn error(&self, msg: &str) {
        println!("{} at line: {}, col: {}", msg, self.line, self.col)
    }
}

pub struct Parser<'a>(Peekable<Lexer<'a>>);

impl<'a> Iterator for Parser<'a> {
    type Item = Val<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.peek() {
            Some(&Token(TokType::LPAREN, _)) => self.parse_list(),
            _ => match self.0.next() {
                Some(Token(tok_type, text)) => match tok_type {
                    TokType::IDENT => Some(Val::Ident(text)),
                    TokType::STRING => Some(Val::String(&text[1..text.len()-1])),
                    TokType::BOOLEAN => match text {
                        "#t" | "#T" => Some(Val::Bool(true)),
                        _ => Some((Val::Bool(false))),
                    },
                    TokType::NUMBER => Some(Val::Num(f64::from_str(text).unwrap())),
                    _ => self.next(),
                },
                None => None,
            }
        }
    }
}

impl<'a> Parser<'a> {
    fn parse_list(&mut self) -> Option<Val<'a>> {
        self.0.next(); // skip open paren
        let mut l: list::List<Box<Val<'a>>> = list::List::new();
        loop {
            match self.0.peek() {
                Some(&Token(TokType::RPAREN, _)) => break,
                Some(_) => match self.next() {
                    Some(v) => l = l.push(Box::new(v)),
                    None => return None,
                },
                None => return None,
            };
        }
        self.0.next(); // skip close paren
        Some(Val::List(l.reverse()))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.res.match_str(&self.input[self.pos..]) {
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
    pub fn match_str<'a>(&self, s: &'a str) -> Option<(TokType, usize)> {
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
}
