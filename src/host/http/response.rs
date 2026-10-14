use crate::bindings::synt::extension::http;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    status: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl Response {
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(found, _)| found.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }

    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }
}

impl From<http::Response> for Response {
    fn from(response: http::Response) -> Self {
        Self {
            status: response.status,
            headers: response
                .headers
                .into_iter()
                .map(|header| (header.name, header.value))
                .collect(),
            body: response.body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Response;
    use crate::bindings::synt::extension::http;

    #[test]
    fn reads_status_headers_and_body() {
        let response = Response::from(http::Response {
            status: 200,
            headers: vec![http::Header {
                name: "content-type".to_owned(),
                value: "text/plain".to_owned(),
            }],
            body: b"ok".to_vec(),
        });

        assert_eq!(response.status(), 200);
        assert_eq!(response.header("Content-Type"), Some("text/plain"));
        assert_eq!(response.text(), "ok");
    }
}
