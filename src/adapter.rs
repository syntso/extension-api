use std::marker::PhantomData;

use crate::bindings::Guest;
use crate::bindings::synt::extension::types;
use crate::event::Event;
use crate::extension::Extension;

pub struct Adapter<E>(PhantomData<E>);

impl<E: Extension> Guest for Adapter<E> {
    fn describe() -> Vec<types::Action> {
        E::describe().into_iter().map(types::Action::from).collect()
    }

    fn run(action: String, target: Option<String>, params: String) -> Result<String, types::Error> {
        E::run(&action, target.as_deref(), &params).map_err(types::Error::from)
    }

    fn start() -> Result<(), types::Error> {
        E::start().map_err(types::Error::from)
    }

    fn render(
        view: String,
        target: Option<String>,
        params: String,
    ) -> Result<String, types::Error> {
        E::render(&view, target.as_deref(), &params).map_err(types::Error::from)
    }

    fn panel_message(panel: String, message: String) -> Result<String, types::Error> {
        E::panel_message(&panel, &message).map_err(types::Error::from)
    }

    fn asset(path: String) -> Option<Vec<u8>> {
        E::asset(&path)
    }

    fn on_event(event: types::Event) -> Result<(), types::Error> {
        E::on_event(&Event::from(event)).map_err(types::Error::from)
    }

    fn on_timer(name: String) -> Result<(), types::Error> {
        E::on_timer(&name).map_err(types::Error::from)
    }

    fn merge(
        kind: String,
        base: Option<Vec<u8>>,
        ours: Vec<u8>,
        theirs: Vec<u8>,
    ) -> Result<Vec<u8>, types::Error> {
        E::merge(&kind, base.as_deref(), &ours, &theirs).map_err(types::Error::from)
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

        fn render(view: &str, target: Option<&str>, _params: &str) -> Result<String, Error> {
            Ok(format!("{view} of {}", target.unwrap_or("nothing")))
        }

        fn asset(path: &str) -> Option<Vec<u8>> {
            (path == "index.html").then(|| b"<p>hi</p>".to_vec())
        }

        fn on_event(event: &Event) -> Result<(), Error> {
            match event.kind() {
                "journal" => Ok(()),
                other => Err(Error::new("unexpected", other)),
            }
        }

        fn merge(
            _kind: &str,
            _base: Option<&[u8]>,
            ours: &[u8],
            theirs: &[u8],
        ) -> Result<Vec<u8>, Error> {
            Ok([ours, theirs].concat())
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

    #[test]
    fn renders_serves_assets_hears_events_and_merges() {
        let rendered = <Adapter<Echo> as Guest>::render(
            "load".to_owned(),
            Some("web-1".to_owned()),
            "{}".to_owned(),
        );
        let event = types::Event {
            kind: "session".to_owned(),
            payload: "{}".to_owned(),
        };

        assert_eq!(rendered.expect("a view"), "load of web-1");
        assert_eq!(
            <Adapter<Echo> as Guest>::asset("index.html".to_owned()),
            Some(b"<p>hi</p>".to_vec())
        );
        assert_eq!(<Adapter<Echo> as Guest>::asset("other".to_owned()), None);
        assert_eq!(
            <Adapter<Echo> as Guest>::on_event(event)
                .expect_err("not heard")
                .code,
            "unexpected"
        );
        assert_eq!(
            <Adapter<Echo> as Guest>::merge("note".to_owned(), None, b"a".to_vec(), b"b".to_vec())
                .expect("merged"),
            b"ab"
        );
    }

    #[test]
    fn defaults_start_quietly_and_refuse_the_rest() {
        assert!(<Adapter<Echo> as Guest>::start().is_ok());
        assert!(<Adapter<Echo> as Guest>::on_timer("tick".to_owned()).is_ok());
        assert_eq!(
            <Adapter<Echo> as Guest>::panel_message("editor".to_owned(), "{}".to_owned())
                .expect_err("no panel")
                .code,
            "unknown_panel"
        );
    }
}
