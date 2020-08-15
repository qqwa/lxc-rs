#[derive(Debug)]
pub enum Error {
    /// Calling into lxc library indicated an error, cause can not be specified
    /// more precise than by proving the function call which returned an error.
    ApiError(String),
    Wrapper(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            Error::ApiError(function) => format!(
                "Call to lxc function \"{}\" indicated an error, please consult documentation",
                function
            ),
            Error::Wrapper(e) => format!("Error: {}", e),
        };
        write!(f, "{}", error_message)
    }
}

impl std::error::Error for Error {}
