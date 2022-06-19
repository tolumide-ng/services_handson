use futures::{Future};
use std::{net::SocketAddr, sync::{Arc, Mutex}};
use hyper::{server::Server, rt::{run}};
use hyper::service::service_fn;
use slab::Slab;


use routes::microservice_handler;

mod response;
mod routes;





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