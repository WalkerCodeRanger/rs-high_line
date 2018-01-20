//use std;
use std::io::StdoutLock;
use std::io::StdinLock;
use std::io::{BufRead, Stdin, Stdout, Write};

pub trait Input<'a, R: BufRead> {
    fn open(&'a mut self) -> R;
}

impl<'a> Input<'a, StdinLock<'a>> for Stdin {
    fn open(&'a mut self) -> StdinLock<'a> {
        return self.lock();
        //return Stdin::lock(self);
    }
}

// impl Input for BufRead {
//     fn open<'a>(&'a mut self) -> Box<BufRead + 'a> {
//         return Box::new(self);
//     }
// }

pub trait Output<'a, W: Write> {
    fn open(&'a mut self) -> W;
}

impl<'a> Output<'a, StdoutLock<'a>> for Stdout {
    fn open(&'a mut self) -> StdoutLock<'a> {
        return self.lock();
    }
}

// impl Output for Write {
//     fn open<'a>(&'a mut self) -> Box<Write + 'a> {
//         return Box::new(self);
//     }
// }
