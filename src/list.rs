use std::boxed::Box;
use std::fmt;
use std::iter;

use list::List::{Cons, Nil};

#[derive(Debug)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        Nil
    }

    pub fn push(self, v: T) -> List<T> {
        Cons(v, Box::new(self))
    }

    pub fn append(self, v: T) -> List<T> {
        match self {
            Cons(w, rest) => Cons(w, Box::new(rest.append(v))),
            Nil => Cons(v, Box::new(Nil))
        }
    }

    pub fn reverse(self) -> List<T> {
        match self {
            Cons(v, rest) => rest.reverse().append(v),
            Nil => Nil,
        }
    }

    pub fn concat(self, l: List<T>) -> List<T> {
        match self {
            Nil => l,
            Cons(v, rest) => Cons(v, Box::new(rest.concat(l))),
        }
    }

    pub fn iter(&self) -> ListIter<T> {
        ListIter{ l: self }
    }
}

impl<T> fmt::Display for List<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut it = self.iter().peekable();
        try!(write!(f, "("));
        loop {
            match it.next() {
                Some(v) => try!(write!(f,"{}", v)),
                None => break,
            };
            match it.peek() {
                Some(_) => try!(write!(f, " ")),
                None => break,
            };
        }
        write!(f, ")")
    }
}

pub struct ListIter<'a, T> where T: 'a {
    l: &'a List<T>,
}

impl<'a, T> iter::Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.l {
            &Cons(ref v, ref rest) => {
                self.l = rest;
                Some(v)
            },
            &Nil => None,
        }
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = ListIter<'a, T>;

    fn into_iter(self) -> ListIter<'a, T> {
        ListIter{ l: self }
    }
}

