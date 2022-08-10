//! Contains [`Error`]
use std::sync::Arc;

/// List of features whose non-activation may cause a runtime error.
/// Used to indicate which lack of feature caused [`Error::FeatureNotActive`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Feature {
    /// Snappy compression and decompression
    Snappy,
    /// Brotli compression and decompression
    Brotli,
    /// Gzip compression and decompression
    Gzip,
    /// Lz4 raw compression and decompression
    Lz4,
    /// Zstd compression and decompression
    Zstd,
}

/// Errors generated by this crate
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    /// General Parquet error.
    General(String),
    /// Error presented when trying to use a code branch that requires a feature.
    FeatureNotActive(Feature, String),
    /// When the parquet file is known to be out of spec.
    OutOfSpec(String),
    /// An error originating from a consumer or dependency
    External(String, Arc<dyn std::error::Error + Send + Sync>),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::General(message) => {
                write!(fmt, "{}", message)
            }
            Error::FeatureNotActive(feature, reason) => {
                write!(
                    fmt,
                    "The feature \"{:?}\" needs to be active to {}",
                    feature, reason
                )
            }
            Error::OutOfSpec(message) => {
                write!(fmt, "{}", message)
            }
            Error::External(message, err) => {
                write!(fmt, "{}: {}", message, err)
            }
        }
    }
}

#[cfg(feature = "snappy")]
impl From<snap::Error> for Error {
    fn from(e: snap::Error) -> Error {
        Error::General(format!("underlying snap error: {}", e))
    }
}

#[cfg(feature = "lz4_flex")]
impl From<lz4_flex::block::DecompressError> for Error {
    fn from(e: lz4_flex::block::DecompressError) -> Error {
        Error::General(format!("underlying lz4_flex error: {}", e))
    }
}

#[cfg(feature = "lz4_flex")]
impl From<lz4_flex::block::CompressError> for Error {
    fn from(e: lz4_flex::block::CompressError) -> Error {
        Error::General(format!("underlying lz4_flex error: {}", e))
    }
}

impl From<parquet_format_safe::thrift::Error> for Error {
    fn from(e: parquet_format_safe::thrift::Error) -> Error {
        Error::General(format!("underlying thrift error: {}", e))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::General(format!("underlying IO error: {}", e))
    }
}

impl From<std::collections::TryReserveError> for Error {
    fn from(e: std::collections::TryReserveError) -> Error {
        Error::General(format!("OOM: {}", e))
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(e: std::num::TryFromIntError) -> Error {
        Error::OutOfSpec(format!("Number must be zero or positive: {}", e))
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(e: std::array::TryFromSliceError) -> Error {
        Error::OutOfSpec(format!("Can't deserialize to parquet native type: {}", e))
    }
}

/// A specialized `Result` for Parquet errors.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! general_err {
    ($fmt:expr) => (Error::General($fmt.to_owned()));
    ($fmt:expr, $($args:expr),*) => (Error::General(format!($fmt, $($args),*)));
    ($e:expr, $fmt:expr) => (Error::General($fmt.to_owned(), $e));
    ($e:ident, $fmt:expr, $($args:tt),*) => (
        Error::General(&format!($fmt, $($args),*), $e));
}
