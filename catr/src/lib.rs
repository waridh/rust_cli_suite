use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;
