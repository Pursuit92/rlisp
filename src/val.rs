use std::fmt;

use std::rc::Rc;

use val::Val::{Ident,Bool,Num,Cons,Nil};

use std::iter;

#[derive(Debug)]
pub enum Val {
    Ident(String),
    Bool(bool),
    Num(f64),
    String(String),
    Cons(Rc<Val>, Rc<Val>),
    Nil
}

impl Val {
    pub fn cons<T, U>(head: T, tail: U) -> Val
        where T: Into<Val>, U: Into<Val> {
            Val::Cons(Rc::new(head.into()), Rc::new(tail.into()))
        }
    pub fn iter(&self) -> Iter {
        Iter{ curr: self, done: false }
    }
}

impl fmt::Display for Val {
    fn fmt(& self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Val::Ident(ref istr) => write!(f, "{}", istr),
            &Val::Bool(b) => write!(f, "{}", b),
            &Val::Num(n) => write!(f, "{}", n),
            &Val::String(ref s) => write!(f, r#""{}""#, s),
            &Val::Cons(_, _) => {
                let mut it = self.iter().peekable();
                try!(write!(f, "("));
                loop {
                    let mut current;
                    match it.next() {
                        Some(v) => {
                            current = v;
                        },
                        None => break,
                    };
                    match it.peek() {
                        Some(&&Nil) => {
                            try!(write!(f, "{}", current));
                            break;
                        },
                        None => {
                            try!(write!(f, ". {}", current));
                            break;
                        },
                        Some(_) => try!(write!(f, "{} ", current))
                    };
                }

                write!(f, ")")
            },
            &Val::Nil => write!(f, "<nil>"),
        }
    }
}

struct Iter<'a> {
    curr: &'a Val,
    done: bool,
}

impl<'a> iter::Iterator for Iter<'a> {
    type Item = &'a Val;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr {
            &Cons(ref v, ref rest) => {
                self.curr = &*rest;
                Some(v)
            },
            &Nil | &Ident(_) | &Bool(_) | &Num(_) | &Val::String(_) => {
                if self.done {
                    None
                } else {
                    self.done = true;
                    Some(self.curr)
                }
            }
        }
    }
}

impl<T> From<T> for Val where T: Into<String> {
    fn from(s: T) -> Val {
        Val::String(s.into())
    }
}

