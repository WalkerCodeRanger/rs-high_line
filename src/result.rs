use PromptResult::*;

pub enum PromptResult<T> {
    Answer(T),
    Error,
    Exit,
}

impl<T> PromptResult<T> {
    pub fn and_then<U, F: FnOnce(T) -> PromptResult<U>>(self, f: F) -> PromptResult<U> {
        match self {
            Answer(a) => f(a),
            // TODO why did Answer require qualification but these don't?
            Error => Error,
            Exit => Exit,
        }
    }
}

impl<T, E> From<Result<T, E>> for PromptResult<T> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(v) => Answer(v),
            Err(_) => Error,
        }
    }
}

impl<T> From<Option<T>> for PromptResult<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Some(v) => Answer(v),
            None => Error,
        }
    }
}
