use crate::action_info::ActionInfo;
use crate::error::Error;
use crate::event::Event;

pub trait Extension {
    fn describe() -> Vec<ActionInfo>;

    fn run(action: &str, target: Option<&str>, params: &str) -> Result<String, Error>;

    fn start() -> Result<(), Error> {
        Ok(())
    }

    fn render(view: &str, _target: Option<&str>, _params: &str) -> Result<String, Error> {
        Err(Error::new("unknown_view", view))
    }

    fn panel_message(panel: &str, _message: &str) -> Result<String, Error> {
        Err(Error::new("unknown_panel", panel))
    }

    fn asset(_path: &str) -> Option<Vec<u8>> {
        None
    }

    fn on_event(_event: &Event) -> Result<(), Error> {
        Ok(())
    }

    fn on_timer(_name: &str) -> Result<(), Error> {
        Ok(())
    }

    fn merge(
        kind: &str,
        _base: Option<&[u8]>,
        _ours: &[u8],
        _theirs: &[u8],
    ) -> Result<Vec<u8>, Error> {
        Err(Error::new(
            "no_merge",
            format!("this extension cannot merge {kind} documents"),
        ))
    }
}
