use crate::bindings::synt::extension::types;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Risk {
    Read,
    Write,
    Privileged,
}

impl From<Risk> for types::Risk {
    fn from(risk: Risk) -> Self {
        match risk {
            Risk::Read => Self::Read,
            Risk::Write => Self::Write,
            Risk::Privileged => Self::Privileged,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_every_risk() {
        assert_eq!(types::Risk::from(Risk::Read), types::Risk::Read);
        assert_eq!(types::Risk::from(Risk::Write), types::Risk::Write);
        assert_eq!(types::Risk::from(Risk::Privileged), types::Risk::Privileged);
    }
}
