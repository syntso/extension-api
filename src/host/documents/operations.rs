use super::{Document, DocumentInfo, Written};
use crate::bindings::synt::extension::documents;
use crate::error::Error;

pub fn listing(project: &str) -> Result<Vec<DocumentInfo>, Error> {
    documents::listing(project)
        .map(|found| found.into_iter().map(DocumentInfo::from).collect())
        .map_err(Error::from)
}

pub fn read(project: &str, path: &str) -> Result<Document, Error> {
    documents::read(project, path)
        .map(Document::from)
        .map_err(Error::from)
}

pub fn write(project: &str, path: &str, body: &[u8], base: Option<&str>) -> Result<Written, Error> {
    documents::write(project, path, body, base)
        .map(Written::from)
        .map_err(Error::from)
}

pub fn remove(project: &str, path: &str) -> Result<(), Error> {
    documents::remove(project, path).map_err(Error::from)
}
