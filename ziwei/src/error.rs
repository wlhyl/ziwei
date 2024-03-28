use std::fmt::Display;

pub enum Error {
    Function(String),
    // 无效的推运时间
    InvalidProcessDateTime(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Error::Function(s) => s,
            Error::InvalidProcessDateTime(s) => s,
        };
        write!(f, "{}", s)
    }
}
