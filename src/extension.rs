use crate::action_info::ActionInfo;
use crate::error::Error;

pub trait Extension {
    fn describe() -> Vec<ActionInfo>;

    fn run(action: &str, target: Option<&str>, params: &str) -> Result<String, Error>;
}
