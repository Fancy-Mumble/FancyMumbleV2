use std::error::Error;

pub mod application_error;
pub mod certificate_error;
pub mod voice_error;

pub type AnyError<T> = Result<T, Box<dyn Error>>;
