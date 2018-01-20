use std::str::FromStr;

pub trait DefaultPrompt: Sized {
    fn parse(String) -> Option<Self>; // Sized needed so we can do this
    const ERROR_PROMPT: &'static str;
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
