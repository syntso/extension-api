use std::marker::PhantomData;

use crate::bindings::Guest;
use crate::bindings::synt::extension::types;
use crate::extension::Extension;

pub struct Adapter<E>(PhantomData<E>);

impl<E: Extension> Guest for Adapter<E> {
    fn describe() -> Vec<types::Action> {
        E::describe().into_iter().map(types::Action::from).collect()
    }

    fn run(action: String, target: Option<String>, params: String) -> Result<String, types::Error> {
        E::run(&action, target.as_deref(), &params).map_err(types::Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action_info::ActionInfo;
    use crate::error::Error;
    use crate::risk::Risk;

    struct Echo;

    impl Extension for Echo {
        fn describe() -> Vec<ActionInfo> {
            vec![ActionInfo::new("echo.say", "Say", Risk::Read).takes_target()]
        }

        fn run(action: &str, target: Option<&str>, params: &str) -> Result<String, Error> {
            match (action, target) {
                ("echo.say", Some(target)) => Ok(format!("{target}: {params}")),
                _ => Err(Error::new("unknown_action", action)),
            }
        }
    }

    #[test]
    fn describes_through_the_interface() {
        let actions = <Adapter<Echo> as Guest>::describe();

        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].id, "echo.say");
        assert!(actions[0].takes_target);
    }

    #[test]
    fn runs_through_the_interface() {
        let output = <Adapter<Echo> as Guest>::run(
            "echo.say".to_owned(),
            Some("web-1".to_owned()),
            "{}".to_owned(),
        )
        .expect("known action");

        assert_eq!(output, "web-1: {}");
    }

    #[test]
    fn passes_errors_through_the_interface() {
        let error = <Adapter<Echo> as Guest>::run("echo.shout".to_owned(), None, "{}".to_owned())
            .expect_err("unknown action");

        assert_eq!(error.code, "unknown_action");
        assert_eq!(error.message, "echo.shout");
    }
}
