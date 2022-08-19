use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    /// The input doesn't contain the specified key.
    #[error("missing key: `{0}`")]
    MissingKey(&'static str),

    /// Could not parse a boolean value.
    #[error("could not parse a boolean value")]
    ParseBoolError(#[from] std::str::ParseBoolError),

    /// Could not parse an integer.
    #[error("could not parse an integer")]
    ParseIntError(#[from] std::num::ParseIntError),

    /// Could not parse a floating-point number.
    #[error("could not parse a floating-point number")]
    ParseFloatError(#[from] std::num::ParseFloatError),

    /// Could not parse a network address.
    #[error("could not parse a network address")]
    AddrParseError(#[from] std::net::AddrParseError),

    #[error("infallible")]
    Infallible(#[from] std::convert::Infallible),
}
