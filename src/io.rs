//use std;
use std::io::StdoutLock;
use std::io::StdinLock;
use std::io::{BufRead, Stdin, Stdout, Write};

pub trait Input<'a, R: BufRead + 'a> {
    fn open<'b>(&'b mut self) -> R
    where
        'b: 'a;
}

impl<'a> Input<'a, StdinLock<'a>> for Stdin {
    fn open<'b>(&'b mut self) -> StdinLock<'b>
    where
        'b: 'a,
    {
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
    fn open<'b>(&'b mut self) -> W
    where
        'b: 'a;
}

impl<'a> Output<'a, StdoutLock<'a>> for Stdout {
    fn open<'b>(&'b mut self) -> StdoutLock<'b>
    where
        'b: 'a,
    {
        return self.lock();
    }
}

// impl Output for Write {
//     fn open<'a>(&'a mut self) -> Box<Write + 'a> {
//         return Box::new(self);
//     }
// }
