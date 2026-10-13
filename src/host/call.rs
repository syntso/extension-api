use crate::bindings;
use crate::error::Error;

pub fn call(action: &str, target: Option<&str>, params: &str) -> Result<String, Error> {
    bindings::call(action, target, params).map_err(Error::from)
}
