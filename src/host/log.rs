use crate::bindings::synt::extension::host;
use crate::level::Level;

pub fn log(level: Level, message: &str) {
    host::log(level.into(), message);
}
