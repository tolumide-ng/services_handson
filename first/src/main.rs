use std::{net::SocketAddr, sync::{Arc, Mutex}, fmt};
use futures::{future, Future};
use hyper::{server::Server, Response, Body, rt::{run}, Request, http::Error, Method, StatusCode};
use hyper::service::service_fn;
use slab::Slab;

type UserId = u64;
struct UserData;
type UserDb = Arc<Mutex<Slab<UserData>>>;

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

fn microservice_handler(req: Request<Body>, user_db: &UserDb) -> impl Future<Item = Response<Body>, Error=Error> {
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
 let user_db = Arc::new(Mutex::new(Slab::new()));

 let server = builder.serve(move|| {
    let user_db = user_db.clone();
    service_fn(move |req| microservice_handler(req, &user_db))
});

 let server = server.map_err(drop);
 run(server);
}