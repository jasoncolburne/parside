pub type BoxedError = Box<dyn std::error::Error>;
pub type Result<T> = core::result::Result<T, BoxedError>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error: {0}")]
    Generic(String),
}

macro_rules! err {
    ($e:expr) => {
        Err(Box::new($e))
    };
}

#[cfg(test)]
mod test {
    use super::{Error, Result};

    fn explode() -> Result<()> {
        err!(Error::Generic("error".to_string()))
    }

    #[test]
    fn err() {
        assert!(explode().is_err());
    }
}
