use crate::bindings;
use crate::level::Level;

pub fn log(level: Level, message: &str) {
    bindings::log(level.into(), message);
}
