use std::io::{stdin, stdout, BufRead, Write};

mod result;
use result::PromptResult;
use result::PromptResult::*;

mod default;
use default::DefaultPromptBuilder;

pub struct PromptBuilder<'a, T> {
    prompt: &'a str,
    parse: Box<Fn(String) -> PromptResult<T> + 'a>,
}

pub fn ask(prompt: &str) -> DefaultPromptBuilder {
    return DefaultPromptBuilder::new(prompt);
}

impl<'a, T: 'a> PromptBuilder<'a, T> {
    pub fn error_prompt_to<R: BufRead, W: Write>(
        self,
        error: &str,
        mut input: R,
        mut output: W,
    ) -> T {
        let mut buffer = String::new();
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
            match (self.parse)(buffer) {
                Answer(value) => {
                    return value;
                }
                Error => {
                    output.write(error.as_bytes()).unwrap();
                    output.write(b"\n").unwrap();
                    output.flush().unwrap();
                    buffer = String::new();
                }
                Exit => {
                    unimplemented!();
                }
            }
        }
    }

    pub fn error_prompt(self, error: &str) -> T {
        let input = stdin();
        let output = stdout();
        return self.error_prompt_to(error, &mut input.lock(), &mut output.lock());
    }

    pub fn parse<U, P: Fn(T) -> Result<U, E> + 'a, E>(
        self,
        parse_value: P,
    ) -> PromptBuilder<'a, U> {
        // destructuring so the compiler knows that only parse needs to live long enough to be used by the closure
        let PromptBuilder { prompt, parse } = self;
        let parse = move |s| parse(s).and_then(|t| parse_value(t).into());
        return PromptBuilder {
            prompt,
            parse: Box::new(parse),
        };
    }

    pub fn transform<U, F: Fn(T) -> Option<U> + 'a>(
        self,
        transform_value: F,
    ) -> PromptBuilder<'a, U> {
        // destructuring so the compiler knows that only parse needs to live long enough to be used by the closure
        let PromptBuilder { prompt, parse } = self;
        // TODO why is the inner closure needed, and how can it be avoided?
        let parse = move |s| parse(s).and_then(|t| transform_value(t).into());
        return PromptBuilder {
            prompt,
            parse: Box::new(parse),
        };
    }

    pub fn validate<F: Fn(&T) -> bool + 'a>(self, validate_value: F) -> PromptBuilder<'a, T> {
        // destructuring so the compiler knows that only parse needs to live long enough to be used by the closure
        let PromptBuilder { prompt, parse } = self;
        let parse =
            move |s| parse(s).and_then(|t| if validate_value(&t) { Answer(t) } else { Error });
        return PromptBuilder {
            prompt,
            parse: Box::new(parse),
        };
    }
}

#[cfg(test)]
mod tests {
    use ask;

    fn setup(input: &[u8]) -> (&[u8], Vec<u8>) {
        return (input, Vec::new());
    }

    fn output_string(output: Vec<u8>) -> String {
        return String::from_utf8(output).expect("Not UTF-8");
    }

    #[test]
    fn ask_for_string() {
        let (input, mut output) = setup(b"My Value\n");

        let value: String = ask("Value?").prompt_to(&input[..], &mut output);

        assert_eq!(output_string(output), "Value? ");
        assert_eq!(value, "My Value")
    }

    #[test]
    fn ask_for_string_with_error_prompt_does_not_accept_empty() {
        let (input, mut output) = setup(b"\nMy Value\n");

        let value: String =
            ask("Value?").error_prompt_to("Please enter a value.", &input[..], &mut output);

        assert_eq!(value, "My Value");
        assert_eq!(
            output_string(output),
            "Value? Please enter a value.\nValue? "
        );
    }

    #[test]
    fn ask_repeats_error_prompt() {
        let (input, mut output) = setup(b"\n\nMy Value\n");

        let value: String = ask("Value?").prompt_to(&input[..], &mut output);

        assert_eq!(
            output_string(output),
            "Value? Please enter a value.\nValue? Please enter a value.\nValue? "
        );
        assert_eq!(value, "My Value")
    }

}
