use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Unexpected error
    Server(anyhow::Error),
    /// Send error to client and disconnect
    Client(anyhow::Error),
}

impl Error {
    pub fn into_inner(self) -> anyhow::Error {
        match self {
            Error::Server(err) => err,
            Error::Client(err) => err,
        }
    }

    pub fn into_client(self) -> Self {
        Self::Client(self.into_inner())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Server(err) => write!(f, "Server error: {err}"),
            Error::Client(err) => write!(f, "Client error: {err}"),
        }
    }
}

// Error are implicitly considered as server errors
impl<E: Into<anyhow::Error>> From<E> for Error {
    fn from(value: E) -> Self {
        Self::Server(value.into())
    }
}

pub trait ContextExt<T> {
    /// Wrap the error value with additional context.
    fn context<C>(self, context: C) -> Result<T>
    where
        C: Display + Send + Sync + 'static;
}

impl<T> ContextExt<T> for Result<T> {
    fn context<C>(self, context: C) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
    {
        match self {
            Ok(x) => Ok(x),
            Err(err) => match err {
                Error::Server(err) => Err(Error::Server(err.context(context))),
                Error::Client(err) => Err(Error::Client(err.context(context))),
            },
        }
    }
}
