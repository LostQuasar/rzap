/// Error type for functions in this crate
#[derive(Debug)]
pub enum Error {
    /// error propagated from reqwest
    Reqwest(reqwest::Error),
    /// error propagated from serde
    Serde(serde_json::Error),
    /// no API token was provided to the API interface
    MissingApiToken,
    /// invalid header value when building the API interface
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reqwest(e) => e.fmt(f),
            Self::Serde(e) => e.fmt(f),
            Self::MissingApiToken => write!(f, "no API token was provided"),
            Self::InvalidHeaderValue(e) => write!(f, "invalid header value for user agent: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Reqwest(e) => e.source(),
            Self::Serde(e) => e.source(),
            Self::MissingApiToken => None,
            Self::InvalidHeaderValue(e) => e.source(),
        }
    }
}
