use super::Picked;
use crate::bindings::synt::extension::files;
use crate::error::Error;

pub fn chosen() -> Vec<Picked> {
    files::chosen().into_iter().map(Picked::from).collect()
}

pub fn read(handle: &str) -> Result<Vec<u8>, Error> {
    files::read(handle).map_err(Error::from)
}

pub fn write(handle: &str, body: &[u8]) -> Result<(), Error> {
    files::write(handle, body).map_err(Error::from)
}
