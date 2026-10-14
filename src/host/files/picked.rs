use super::Access;
use crate::bindings::synt::extension::files;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Picked {
    pub handle: String,
    pub name: String,
    pub access: Access,
}

impl From<files::Picked> for Picked {
    fn from(picked: files::Picked) -> Self {
        Self {
            handle: picked.handle,
            name: picked.name,
            access: picked.access.into(),
        }
    }
}
