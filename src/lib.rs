use std::str::FromStr;
use std::io::{stdin, stdout, BufRead, Write};

pub struct PromptBuilder<'a, T> {
    prompt: &'a str,
    parse: Box<Fn(String) -> Option<T> + 'a>,
}

pub trait DefaultPrompt: Sized {
    fn parse(String) -> Option<Self>;
    const ERROR_PROMPT: &'static str;
}

fn string_identity(value: String) -> Option<String> {
    return Some(value);
}

pub fn ask(prompt: &str) -> PromptBuilder<String> {
    return PromptBuilder {
        prompt,
        parse: Box::new(&string_identity),
    };
}

impl<'a, T: 'a> PromptBuilder<'a, T> {
    pub fn error_prompt_to(self, error: &str, input: &mut BufRead, output: &mut Write) -> T {
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

    pub fn parse<U, P: Fn(T) -> Result<U, E> + 'a, E>(
        self,
        parse_value: P,
    ) -> PromptBuilder<'a, U> {
        // destructuring so the compiler knows that only parse needs to live long enough to be used by the closure
        let PromptBuilder { prompt, parse } = self;
        let parse = move |s| parse(s).and_then(|t| parse_value(t).ok());
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
        let parse = move |s| parse(s).and_then(|t| transform_value(t));
        return PromptBuilder {
            prompt,
            parse: Box::new(parse),
        };
    }

    pub fn validate<F: Fn(&T) -> bool + 'a>(self, validate_value: F) -> PromptBuilder<'a, T> {
        // destructuring so the compiler knows that only parse needs to live long enough to be used by the closure
        let PromptBuilder { prompt, parse } = self;
        let parse = move |s| parse(s).and_then(|t| if validate_value(&t) { Some(t) } else { None });
        return PromptBuilder {
            prompt,
            parse: Box::new(parse),
        };
    }
}

impl<'a> PromptBuilder<'a, String> {
    pub fn prompt<T: DefaultPrompt>(self) -> T {
        return self.transform(T::parse).error_prompt(T::ERROR_PROMPT);
    }

    pub fn parse_as<T: DefaultPrompt + 'a>(self) -> PromptBuilder<'a, T> {
        return self.transform(T::parse);
    }
}

impl DefaultPrompt for String {
    fn parse(value: String) -> Option<Self> {
        return if value.is_empty() { None } else { Some(value) };
    }
    const ERROR_PROMPT: &'static str = "Please enter a value.";
}

impl DefaultPrompt for Option<String> {
    fn parse(value: String) -> Option<Self> {
        return Some(Some(value));
    }
    const ERROR_PROMPT: &'static str = "<Can't Fail>";
}

impl DefaultPrompt for u64 {
    fn parse(value: String) -> Option<Self> {
        return u64::from_str(&value).ok();
    }
    const ERROR_PROMPT: &'static str = "Please enter a non-negative number.";
}

#[cfg(test)]
mod tests {
    //use {ask};

    // #[test]
    // fn ask_has_default_error_prompt() {
    //     assert_eq!(ask("?").error_prompt, "Please enter a value.");
    // }

    // #[test]
    // fn play() {
    //     let b = Builder {};
    //     b.parse_as();
    // }
}
