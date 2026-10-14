use crate::bindings::synt::extension::host;
use crate::error::Error;

pub fn call(action: &str, target: Option<&str>, params: &str) -> Result<String, Error> {
    host::call(action, target, params).map_err(Error::from)
}
