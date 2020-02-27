use std::fmt;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("HTTP error")]
    Http(#[source] HttpError),
    #[error("HTTP status")]
    HttpStatus(#[source] HttpStatusError),
}

#[derive(Debug, thiserror::Error)]
pub struct HttpError(#[source] http::Error);

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<http::Error> for Error {
    fn from(e: http::Error) -> Self {
        Error::Http(HttpError(e))
    }
}

#[derive(Debug, thiserror::Error)]
pub struct HttpStatusError(http::StatusCode);

impl fmt::Display for HttpStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<http::StatusCode> for Error {
    fn from(e: http::StatusCode) -> Self {
        Error::HttpStatus(HttpStatusError(e))
    }
}
