use std::boxed::Box;
use std::error::Error;
use std::marker::{Send, Sync};

pub type BoxError = Box<dyn Error + Send + Sync>;
pub type BoxResult<T> = Result<T, BoxError>;