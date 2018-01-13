//use std;
use std::io::{BufRead, Stdin, Stdout, Write};

pub trait Input {
    fn open<'a>(&'a mut self) -> Box<BufRead + 'a>;
}

impl Input for Stdin {
    fn open<'a>(&'a mut self) -> Box<BufRead + 'a> {
        return Box::new(self.lock());
    }
}

impl Input for BufRead {
    fn open<'a>(&'a mut self) -> Box<BufRead + 'a> {
        return Box::new(self);
    }
}

pub trait Output {
    fn open<'a>(&'a mut self) -> Box<Write + 'a>;
}

impl Output for Stdout {
    fn open<'a>(&'a mut self) -> Box<Write + 'a> {
        return Box::new(self.lock());
    }
}

impl Output for Write {
    fn open<'a>(&'a mut self) -> Box<Write + 'a> {
        return Box::new(self);
    }
}
