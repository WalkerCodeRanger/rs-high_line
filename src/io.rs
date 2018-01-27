use std::io::{BufRead, Stdin, Stdout, Write};

pub trait Input {
    fn open(&mut self) -> impl BufRead;
}

impl Input for Stdin {
    fn open(&mut self) -> impl BufRead {
        return self.lock();
    }
}

// impl Input for BufRead {
//     fn open<'a>(&'a mut self) -> Box<BufRead + 'a> {
//         return Box::new(self);
//     }
// }

pub trait Output {
    fn open(&mut self) -> impl Write;
}

impl Output for Stdout {
    fn open(&mut self) -> impl Write {
        return self.lock();
    }
}

// impl Output for Write {
//     fn open<'a>(&'a mut self) -> Box<Write + 'a> {
//         return Box::new(self);
//     }
// }
