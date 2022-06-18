use std::net::SocketAddr;

use hyper::{server::Server, service::service_fn_ok, Response, Body, rt::{Future, run}};

fn main() {
 let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();
 let builder = Server::bind(&addr);

 let server = builder.serve(|| {
    service_fn_ok(|_| {
        Response::new(Body::from("Almost microservice..."))
    })
 });

 let server = server.map_err(drop);
 run(server);
}