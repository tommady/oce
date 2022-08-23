mod errors;
mod rest;
mod schema;

pub type Result<T> = std::result::Result<T, errors::FtxError>;
