use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("missing key: `{0}`")]
    MissingKey(&'static str),

    #[error("could not parse an integer")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("could not parse a floating-point number")]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error("infallible")]
    Infallible(#[from] std::convert::Infallible),
}
