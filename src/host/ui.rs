use crate::bindings::synt::extension::ui;
use crate::error::Error;

pub fn patch_view(view: &str, patch: &str) -> Result<(), Error> {
    ui::patch_view(view, patch).map_err(Error::from)
}

pub fn post(panel: &str, message: &str) -> Result<(), Error> {
    ui::post(panel, message).map_err(Error::from)
}
