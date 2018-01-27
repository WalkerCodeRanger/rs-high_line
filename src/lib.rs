#![feature(conservative_impl_trait)]
use std::io::{stdin, stdout, BufRead, Write};

mod default;
use default::DefaultPromptBuilder;

pub struct PromptBuilder<'a, T, P: Fn(String) -> Option<T>> {
    prompt: &'a str,
    parse: P,
}

pub fn ask(prompt: &str) -> DefaultPromptBuilder {
    return DefaultPromptBuilder::new(prompt);
}

impl<'a, T, P: Fn(String) -> Option<T>> PromptBuilder<'a, T, P> {
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
                Some(value) => {
                    return value;
                }
                None => {
                    output.write(error.as_bytes()).unwrap();
                    output.write(b"\n").unwrap();
                    output.flush().unwrap();
                    buffer = String::new();
                }
            }
        }
    }

    pub fn error_prompt(self, error: &str) -> T {
        let input = stdin();
        let output = stdout();
        return self.error_prompt_to(error, &mut input.lock(), &mut output.lock());
    }

    pub fn parse<U, F: Fn(T) -> Result<U, E> + 'a, E>(
        self,
        parse_value: F,
    ) -> PromptBuilder<'a, U, impl Fn(String) -> Option<U>> {
        // destructuring so the compiler knows that only parse needs to live long enough to be used by the closure
        let PromptBuilder { prompt, parse } = self;
        let parse = move |s| parse(s).and_then(|t| parse_value(t).ok());
        return PromptBuilder {
            prompt,
            parse: parse,
        };
    }

    pub fn transform<U, F: Fn(T) -> Option<U> + 'a>(
        self,
        transform_value: F,
    ) -> PromptBuilder<'a, U, impl Fn(String) -> Option<U>> {
        // destructuring so the compiler knows that only parse needs to live long enough to be used by the closure
        let PromptBuilder { prompt, parse } = self;
        // TODO why is the inner closure needed, and how can it be avoided?
        let parse = move |s| parse(s).and_then(|t| transform_value(t));
        return PromptBuilder {
            prompt,
            parse: parse,
        };
    }

    pub fn validate<F: Fn(&T) -> bool + 'a>(
        self,
        validate_value: F,
    ) -> PromptBuilder<'a, T, impl Fn(String) -> Option<T>> {
        // destructuring so the compiler knows that only parse needs to live long enough to be used by the closure
        let PromptBuilder { prompt, parse } = self;
        let parse = move |s| parse(s).and_then(|t| if validate_value(&t) { Some(t) } else { None });
        return PromptBuilder {
            prompt,
            parse: parse,
        };
    }

    pub fn default_on(self, value: &'a str) -> PromptBuilder<'a, T, impl Fn(String) -> Option<T>>
    where
        T: Default,
    {
        // destructuring so the compiler knows that only parse needs to live long enough to be used by the closure
        let PromptBuilder { prompt, parse } = self;
        let parse = move |s| {
            if s == value {
                Some(T::default())
            } else {
                parse(s)
            }
        };
        return PromptBuilder {
            prompt,
            parse: parse,
        };
    }

    pub fn exit_on(
        self,
        value: &'a str,
    ) -> PromptBuilder<'a, Option<T>, impl Fn(String) -> Option<Option<T>>> {
        // destructuring so the compiler knows that only parse needs to live long enough to be used by the closure
        let PromptBuilder { prompt, parse } = self;
        let parse = move |s| {
            if s == value {
                Some(None) // Not None because that would mean error, we have a value, it is None
            } else {
                Some(parse(s))
            }
        };
        return PromptBuilder {
            prompt,
            parse: parse,
        };
    }

    // TODO implement exit_with(self, value: &'a str, result: &'a T)
}

impl<'a, T: 'a, P: Fn(String) -> Option<Option<T>>> PromptBuilder<'a, Option<T>, P> {
    pub fn and_on(
        self,
        value: &'a str,
    ) -> PromptBuilder<'a, Option<T>, impl Fn(String) -> Option<Option<T>>> {
        self.default_on(value)
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
        assert_eq!(value, "My Value");
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
        assert_eq!(value, "My Value");
    }

    #[test]
    fn exit_on_exits() {
        let (input, mut output) = setup(b"n\n");
        let value: Option<u64> = ask("Number, n to exit?")
            .parse_as::<u64>()
            .exit_on("n")
            .error_prompt_to("Please enter a number", &input[..], &mut output);

        assert_eq!(output_string(output), "Number, n to exit? ");
        assert_eq!(value, None);
    }
}
