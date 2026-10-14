use crate::bindings::synt::extension::timers;
use crate::error::Error;

pub fn set(name: &str, every_seconds: u32) -> Result<(), Error> {
    timers::set(name, every_seconds).map_err(Error::from)
}

pub fn clear(name: &str) {
    timers::clear(name);
}
