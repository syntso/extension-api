use synt_extension_api::{ActionInfo, Error, Extension, Level, Risk, register_extension};

struct Hello;

impl Extension for Hello {
    fn describe() -> Vec<ActionInfo> {
        let greet = ActionInfo::new("hello.greet", "Greet a host", Risk::Read)
            .takes_target()
            .params_schema(r#"{"type":"object","properties":{"name":{"type":"string"}}}"#);

        vec![greet]
    }

    fn run(action: &str, target: Option<&str>, params: &str) -> Result<String, Error> {
        if action != "hello.greet" {
            return Err(Error::new("unknown_action", action));
        }

        let target = target.ok_or_else(|| Error::new("missing_target", "pick a host"))?;
        synt_extension_api::log(Level::Info, &format!("greeting {target}"));

        let uptime = synt_extension_api::call("ssh.exec", Some(target), r#"{"command":"uptime"}"#)?;
        let count = synt_extension_api::storage::get("count").map_or(0, |bytes| bytes.len());
        synt_extension_api::storage::set("count", &vec![0; count + 1])?;

        Ok(format!("hello {target} with {params}: {uptime}"))
    }
}

register_extension!(Hello);
