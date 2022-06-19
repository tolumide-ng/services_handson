use std::net::SocketAddr;
use hyper::{Server, service::service_fn_ok, Response, Body, rt::Future};
use log::{debug, info, trace, logger};

use pretty_env_logger;
// #[macro_use] extern crate log;

fn main() {
    pretty_env_logger::init();

    info!("Rand Microservice - v0.1.0");
    trace!("Starting...");
    let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();
    debug!("Trying to bind server to address: {}", addr);

    let builder = Server::bind(&addr);
    trace!("Creating service handler...");

    let server = builder.serve(|| {
        service_fn_ok(|req| {
            trace!("Incoming request is: {:?}", req);
            let random_byte = rand::random::<u8>();
            debug!("Generated value is: {}", random_byte);
            Response::new(Body::from(random_byte.to_string()))
        })
    });
    info!("Used address: {}", server.local_addr());
    let server = server.map_err(drop);
    debug!("Run!");
    hyper::rt::run(server);
}
