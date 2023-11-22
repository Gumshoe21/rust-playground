use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn test_parse(val: &str) {
    unimplemented!();
}
