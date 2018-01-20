use result::PromptResult::*;
use std::io::Write;
use std::io::BufRead;
use PromptBuilder;
use std::str::FromStr;

pub struct DefaultPromptBuilder<'a> {
    prompt: &'a str,
}

pub trait DefaultPrompt: Sized {
    fn parse(String) -> Option<Self>; // Sized needed so we can do this
    const ERROR_PROMPT: &'static str;
}

impl<'a> DefaultPromptBuilder<'a> {
    pub fn new(prompt: &'a str) -> Self {
        return DefaultPromptBuilder { prompt };
    }

    pub fn error_prompt_to<T: DefaultPrompt, R: BufRead, W: Write>(
        self,
        error: &str,
        input: R,
        output: W,
    ) -> T {
        return self.parse_as().error_prompt_to(error, input, output);
    }

    pub fn error_prompt<T: DefaultPrompt>(self, error: &str) -> T {
        return self.parse_as().error_prompt(error);
    }

    pub fn prompt_to<T: DefaultPrompt, R: BufRead, W: Write>(self, input: R, output: W) -> T {
        return self.parse_as::<T>()
            .error_prompt_to(T::ERROR_PROMPT, input, output);
    }

    pub fn prompt<T: DefaultPrompt>(self) -> T {
        return self.parse_as::<T>().error_prompt(T::ERROR_PROMPT);
    }

    pub fn parse_as<T: DefaultPrompt + 'a>(self) -> PromptBuilder<'a, T> {
        let parse = move |s| T::parse(s).into();
        return PromptBuilder {
            prompt: self.prompt,
            parse: Box::new(parse),
        };
    }

    pub fn parse<U, P: Fn(String) -> Result<U, E> + 'a, E>(
        self,
        parse_value: P,
    ) -> PromptBuilder<'a, U> {
        let parse = move |s| parse_value(s).into();
        return PromptBuilder {
            prompt: self.prompt,
            parse: Box::new(parse),
        };
    }

    pub fn transform<U, F: Fn(String) -> Option<U> + 'a>(
        self,
        transform_value: F,
    ) -> PromptBuilder<'a, U> {
        let parse = move |s| transform_value(s).into();
        return PromptBuilder {
            prompt: self.prompt,
            parse: Box::new(parse),
        };
    }

    pub fn validate<F: Fn(&String) -> bool + 'a>(
        self,
        validate_value: F,
    ) -> PromptBuilder<'a, String> {
        let parse = move |s| if validate_value(&s) { Answer(s) } else { Error };
        return PromptBuilder {
            prompt: self.prompt,
            parse: Box::new(parse),
        };
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
