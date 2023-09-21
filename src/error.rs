use anyhow::anyhow;
use std::fmt::{Debug, Formatter, Display};
use std::error::Error;

pub type NnResult<T> = std::result::Result<T, NnError>;

pub struct NnError {
    e : anyhow::Error,
}

impl NnError {
    pub fn new(e : impl Error + Send + Sync + 'static) -> Self{ Self{ e : e.into() } }
    pub fn to_string(&self) -> String{ self.e.to_string() }
}

impl Debug for NnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.e, f)
    }
}

impl Display for NnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.e, f)
    }
}

impl Error for NnError{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.e.source()
    }
}

impl From<std::io::Error> for NnError {
    fn from(e : std::io::Error) -> Self {
        Self::new(e)
    }
}

impl From<async_channel::RecvError> for NnError{
    fn from(e : async_channel::RecvError) -> Self {
        Self{ e : e.into() }
    }
}

impl From<String> for NnError {
    fn from(s : String) -> Self {
        Self{ e : anyhow!("{}", s) }
    }
}

impl From<&str> for NnError {
    fn from(s : &str) -> Self {
        Self{ e : anyhow!("{}", s) }
    }
}



