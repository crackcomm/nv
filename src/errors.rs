//! Application error types.

#[derive(Debug, err_derive::Error)]
pub enum Error {
    #[error(display = "Abort!")]
    Abort,

    #[error(display = "Whoops, divided by zero!")]
    DivideByZero,

    #[error(display = "{}", _0)]
    Zbox(#[error(from)] zbox::Error),

    #[error(display = "{}", _0)]
    Repl(#[error(from)] repl_rs::Error),

    #[error(display = "I/O: {}", _0)]
    Io(#[error(source, from)] std::io::Error),

    #[error(display = "{}", _0)]
    PathAbs(#[error(source, from)] path_abs::Error),

    #[error(display = "File exists: {}", _0)]
    FileExists(String),

    #[error(display = "Not a directory: {}", _0)]
    NotDirectory(String),

    #[error(display = "Clipboard error: {:?}", _0)]
    Clipboard(Box<dyn std::error::Error>),

    #[error(display = "Utf8 error: {:?}", _0)]
    Utf8(#[error(source, from)] std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, Error>;
