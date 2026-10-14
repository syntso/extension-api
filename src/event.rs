use crate::bindings::synt::extension::types;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    kind: String,
    payload: String,
}

impl Event {
    pub fn new(kind: impl Into<String>, payload: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            payload: payload.into(),
        }
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn payload(&self) -> &str {
        &self.payload
    }
}

impl From<types::Event> for Event {
    fn from(event: types::Event) -> Self {
        Self::new(event.kind, event.payload)
    }
}
