use std::net::SocketAddr;

use hyper::{server::Server, service::{service_fn_ok, service_fn}, Response, Body, rt::{run}, Request, http::Error, Method, StatusCode};
use futures::{future, Future};

const INDEX: &'static str = r#"
<!doctype html>
<html>
    <head>
        <title>Rust Microservice</title>
    </head>
    <body>
        <h3>Rust Microservice
    </body>
</html>
"#;

fn microservice_handler(req: Request<Body>) -> impl Future<Item = Response<Body>, Error=Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            future::ok(Response::new(INDEX.into()))
        },
        _ => {
            let response = Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()).unwrap();
            future::ok(response)
        }
    }
}

fn main() {
 let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();
 let builder = Server::bind(&addr);

 let server = builder.serve(|| service_fn(microservice_handler));

 let server = server.map_err(drop);
 run(server);
}