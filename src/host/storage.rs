use crate::bindings::synt::extension::storage;
use crate::error::Error;

pub fn get(key: &str) -> Option<Vec<u8>> {
    storage::get(key)
}

pub fn set(key: &str, value: &[u8]) -> Result<(), Error> {
    storage::set(key, value).map_err(Error::from)
}
