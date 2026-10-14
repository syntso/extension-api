use crate::bindings::synt::extension::http;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    secrets: Vec<(String, String, String)>,
    body: Option<Vec<u8>>,
    timeout_ms: Option<u32>,
}

impl Request {
    pub fn new(method: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            method: method.into(),
            url: url.into(),
            headers: Vec::new(),
            secrets: Vec::new(),
            body: None,
            timeout_ms: None,
        }
    }

    pub fn get(url: impl Into<String>) -> Self {
        Self::new("GET", url)
    }

    pub fn post(url: impl Into<String>) -> Self {
        Self::new("POST", url)
    }

    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    #[must_use]
    pub fn secret_header(
        mut self,
        name: impl Into<String>,
        secret: impl Into<String>,
        prefix: impl Into<String>,
    ) -> Self {
        self.secrets
            .push((name.into(), secret.into(), prefix.into()));
        self
    }

    #[must_use]
    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = Some(body.into());
        self
    }

    #[must_use]
    pub fn timeout_ms(mut self, timeout_ms: u32) -> Self {
        self.timeout_ms = Some(timeout_ms);
        self
    }
}

impl From<Request> for http::Request {
    fn from(request: Request) -> Self {
        let headers = request
            .headers
            .into_iter()
            .map(|(name, value)| http::Header { name, value })
            .collect();
        let secrets = request
            .secrets
            .into_iter()
            .map(|(name, secret, prefix)| http::SecretHeader {
                name,
                secret,
                prefix,
            })
            .collect();

        Self {
            method: request.method,
            url: request.url,
            headers,
            secrets,
            body: request.body,
            timeout_ms: request.timeout_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::bindings::synt::extension::http;

    #[test]
    fn carries_headers_secrets_and_body_to_the_interface() {
        let request = Request::post("https://grafana.example.com/api/ds/query")
            .header("Accept", "application/json")
            .secret_header("Authorization", "grafana/token", "Bearer ")
            .body("{}")
            .timeout_ms(5_000);

        let wire = http::Request::from(request);

        assert_eq!(wire.method, "POST");
        assert_eq!(wire.headers[0].name, "Accept");
        assert_eq!(wire.secrets[0].secret, "grafana/token");
        assert_eq!(wire.secrets[0].prefix, "Bearer ");
        assert_eq!(wire.body.as_deref(), Some(&b"{}"[..]));
        assert_eq!(wire.timeout_ms, Some(5_000));
    }
}
