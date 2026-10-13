use crate::bindings;
use crate::error::Error;

pub fn get(key: &str) -> Option<Vec<u8>> {
    bindings::storage_get(key)
}

pub fn set(key: &str, value: &[u8]) -> Result<(), Error> {
    bindings::storage_set(key, value).map_err(Error::from)
}
