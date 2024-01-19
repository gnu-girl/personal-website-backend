use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))] 
pub enum Error {
    #[snafu(display("Item with id={} not found", id))]
    InvalidId {
        id: i32,
        source: diesel::result::Error
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;