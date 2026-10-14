use super::DocumentInfo;
use crate::bindings::synt::extension::documents;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Written {
    pub info: DocumentInfo,
    pub merged: bool,
}

impl From<documents::Written> for Written {
    fn from(written: documents::Written) -> Self {
        Self {
            info: written.info.into(),
            merged: written.merged,
        }
    }
}
