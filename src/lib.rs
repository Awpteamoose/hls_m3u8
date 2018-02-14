#[macro_use]
extern crate trackable;

pub use error::{Error, ErrorKind};

pub mod attribute;
pub mod master_playlist;
pub mod media_playlist;
pub mod media_segment;
pub mod string;
pub mod tag;
pub mod value;
pub mod version;

mod error;
pub mod line;

pub type Result<T> = std::result::Result<T, Error>;
