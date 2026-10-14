use crate::bindings::synt::extension::files;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    Read,
    ReadWrite,
}

impl From<files::Access> for Access {
    fn from(access: files::Access) -> Self {
        match access {
            files::Access::Read => Self::Read,
            files::Access::ReadWrite => Self::ReadWrite,
        }
    }
}
