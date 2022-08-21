use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("\nUsage:\ndomain-scanner <example.com>\ndomain-scanner -d <example.com> --ports")]
    CliUsage,
    #[error("Request: {0}")]
    Request(String),
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Request(err.to_string())
    }
}
