use super::DocumentInfo;
use crate::bindings::synt::extension::documents;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub info: DocumentInfo,
    pub body: Vec<u8>,
}

impl From<documents::Document> for Document {
    fn from(document: documents::Document) -> Self {
        Self {
            info: document.info.into(),
            body: document.body,
        }
    }
}
