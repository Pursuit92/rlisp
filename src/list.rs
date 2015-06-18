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

    pub fn pop(self) -> (Option<T>, List<T>) {
        match self {
            Cons(v, rest) => {
                (Some(v), *rest)
            },
            _ => (None, Nil),
        }
    }

    pub fn append(self, v: T) -> List<T> {
        match self {
            Cons(w, rest) => Cons(w, Box::new(rest.append(v))),
            Nil => Cons(v, Box::new(Nil))
        }
    }

    pub fn reverse(self) -> List<T> {
        let mut stack = List::new();
        let mut popped = (None, self);
        loop {
            popped = popped.1.pop();
            match popped.0 {
                None => break,
                Some(v) => stack = stack.push(v),
            };
        }
        stack
    }

    pub fn concat(self, l: List<T>) -> List<T> {
        match self {
            Nil => l,
            Cons(v, rest) => Cons(v, Box::new(rest.concat(l))),
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter{ l: self }
    }
    pub fn owned_iter(&self) -> OwnedIter<T> where T: Clone {
        OwnedIter{ i: self.iter() }
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


pub struct Iter<'a, T> where T: 'a {
    l: &'a List<T>,
}

impl<'a, T> iter::Iterator for Iter<'a, T> {
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
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        Iter{ l: self }
    }
}

pub struct OwnedIter<'a, T> where T: 'a + Clone {
    i: Iter<'a, T>,
}

impl<'a, T> iter::Iterator for OwnedIter<'a, T> where T: Clone {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.i.next() {
            None => None,
            Some(v) => Some(v.clone()),
        }
    }
}

pub trait OwnedIterator {
    type Item;

    fn next_owned(&mut self) -> Option<Self::Item>;
}

impl<I> OwnedIterator for I where I: Iterator, I::Item: Clone {
    type Item = I::Item;

    fn next_owned(&mut self) -> Option<Self::Item> where Self::Item: Clone {
        match self.next() {
            None => None,
            Some(ref v) => Some(v.clone()),
        }
    }
}
