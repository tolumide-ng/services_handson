use hyper::{StatusCode, Response, Body};

pub fn response_with_code(status_code: StatusCode) -> Response<Body> {
    Response::builder().status(status_code).body(Body::empty()).unwrap()
}