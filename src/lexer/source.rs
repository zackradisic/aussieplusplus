use super::super::upside_down::rightside_up;
use std::{iter::Rev, str::Chars};

use itertools::{multipeek, MultiPeek};

pub trait Source {
    fn reset_peek(&mut self);
    fn peek(&mut self) -> Option<&char>;
}

pub struct Regular<'a> {
    src: MultiPeek<Chars<'a>>,
}

impl<'a> Regular<'a> {
    pub fn new(src: Chars<'a>) -> Self {
        Self {
            src: multipeek(src),
        }
    }
}

impl<'a> Source for Regular<'a> {
    fn reset_peek(&mut self) {
        self.src.reset_peek()
    }

    fn peek(&mut self) -> Option<&char> {
        self.src.peek()
    }
}

impl<'a> Iterator for Regular<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.src.next()
    }
}

pub struct UpsideDown<'a> {
    src: MultiPeek<Rev<Chars<'a>>>,
    dummy: Option<char>,
}

impl<'a> UpsideDown<'a> {
    pub fn new(src: Chars<'a>) -> Self {
        Self {
            src: multipeek(src.rev()),
            dummy: None,
        }
    }
}

impl<'a> Source for UpsideDown<'a> {
    fn reset_peek(&mut self) {
        self.src.reset_peek()
    }

    fn peek(&mut self) -> Option<&char> {
        match self.src.peek() {
            None => None,
            Some(v) => {
                let _ = self.dummy.insert(rightside_up(*v));
                self.dummy.as_ref().take()
            }
        }
    }
}

impl<'a> Iterator for UpsideDown<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.src.next() {
            None => None,
            Some(v) => Some(rightside_up(v)),
        }
    }
}
