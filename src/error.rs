/// Result alias with a custom error type.
pub type Result<T> = std::result::Result<T, IpfscidError>;

/// Custom error type
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum IpfscidError {
    #[error("Other: {}", &.0)]
    AdHoc(String),

    #[error("{msg}: {source:?}")]
    Compat {
        msg: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },

    #[error("No last chunk")]
    NoLastChunk,

    #[error("Invalid UTF-8")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),

    #[error("Invalid CID")]
    InvalidCid(#[from] cid::Error),

    #[error("Invalid multihash")]
    InvalidMultihash(#[from] cid::multihash::Error),
}

impl IpfscidError {
    /// Create a new error from a string.
    pub fn new(msg: impl Into<String>) -> Self {
        Self::AdHoc(msg.into())
    }

    /// Create a new error from a string and a source error.
    pub fn with_source(
        msg: impl Into<String>,
        source: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    ) -> Self {
        Self::Compat { msg: msg.into(), source: source.into() }
    }
}
