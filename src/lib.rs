use std::marker::PhantomData;
use std::io::StdoutLock;
use std::io::StdinLock;
use std::io::{stdin, stdout, BufRead, Stdin, Stdout, Write};

mod io;
use io::{Input, Output};

pub struct PromptBuilder<'a, R: BufRead, I: Input<'a, R>, W: Write, O: Output<'a, W>> {
    input: I,
    output: O,
    prompt: &'a str,
    error_prompt: &'a str,
    _reader: PhantomData<R>,
    _writer: PhantomData<W>,
}

type StdPromptBuilder<'a> = PromptBuilder<'a, StdinLock<'a>, Stdin, StdoutLock<'a>, Stdout>;

pub fn ask(prompt: &str) -> StdPromptBuilder {
    return PromptBuilder {
        input: stdin(),
        output: stdout(),
        prompt,
        error_prompt: "Please enter a value.",
        _reader: PhantomData,
        _writer: PhantomData,
    };
}

impl<'a, R: BufRead, I: Input<'a, R> + 'a, W: Write, O: Output<'a, W> + 'a>
    PromptBuilder<'a, R, I, W, O> {
    pub fn prompt(mut self) -> String {
        let mut buffer = String::new();
        let mut input = self.input.open();
        let mut output = self.output.open();
        loop {
            output.write(self.prompt.as_bytes()).unwrap();
            output.write(b" ").unwrap();
            output.flush().unwrap();

            input.read_line(&mut buffer).unwrap();
            // TODO why isn't there a better way to read a line without the newline?
            if buffer.ends_with("\r\n") {
                buffer.pop(); // remove extra newline char
            }
            buffer.pop(); // remove newline
            if buffer.is_empty() {
                output.write(self.error_prompt.as_bytes()).unwrap();
                output.write(b"\n").unwrap();
                output.flush().unwrap();
            } else {
                return buffer;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ask;

    #[test]
    fn ask_has_default_error_prompt() {
        assert_eq!(ask("?").error_prompt, "Please enter a value.");
    }
}
