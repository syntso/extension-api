use super::{Request, Response};
use crate::bindings::synt::extension::http;
use crate::error::Error;

pub fn send(request: Request) -> Result<Response, Error> {
    http::send(&request.into())
        .map(Response::from)
        .map_err(Error::from)
}
