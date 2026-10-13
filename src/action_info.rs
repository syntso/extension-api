use crate::bindings::synt::extension::types;
use crate::risk::Risk;

const NO_PARAMS: &str = r#"{"type":"object","properties":{}}"#;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionInfo {
    id: String,
    title: String,
    risk: Risk,
    takes_target: bool,
    params_schema: String,
}

impl ActionInfo {
    pub fn new(id: impl Into<String>, title: impl Into<String>, risk: Risk) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            risk,
            takes_target: false,
            params_schema: NO_PARAMS.to_owned(),
        }
    }

    #[must_use]
    pub fn takes_target(mut self) -> Self {
        self.takes_target = true;
        self
    }

    #[must_use]
    pub fn params_schema(mut self, schema: impl Into<String>) -> Self {
        self.params_schema = schema.into();
        self
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn risk(&self) -> Risk {
        self.risk
    }
}

impl From<ActionInfo> for types::Action {
    fn from(info: ActionInfo) -> Self {
        Self {
            id: info.id,
            title: info.title,
            risk: info.risk.into(),
            takes_target: info.takes_target,
            params_schema: info.params_schema,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_no_target_and_no_params() {
        let action = types::Action::from(ActionInfo::new("uptime.check", "Check", Risk::Read));

        assert!(!action.takes_target);
        assert_eq!(action.params_schema, NO_PARAMS);
        assert_eq!(action.risk, types::Risk::Read);
    }

    #[test]
    fn carries_target_and_schema() {
        let schema = r#"{"type":"object","properties":{"unit":{"type":"string"}}}"#;

        let info = ActionInfo::new("uptime.check", "Check uptime", Risk::Write)
            .takes_target()
            .params_schema(schema);
        let action = types::Action::from(info);

        assert_eq!(action.id, "uptime.check");
        assert_eq!(action.title, "Check uptime");
        assert!(action.takes_target);
        assert_eq!(action.params_schema, schema);
    }
}
