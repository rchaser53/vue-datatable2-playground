#[macro_use]
extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;
extern crate serde_json;

use std::io::{self, Write};
use futures::future::{Future};
use futures::stream::{Stream};
use hyper::{Client, Method};
use tokio_core::reactor::Core;
use serde_json::{Value};

use hyper::header::{Headers, UserAgent, ContentLength};
use hyper::server::{Http, Request, Response, Service};
use hyper_tls::HttpsConnector;

const PHRASE: &'static str = "Hello, World!";

struct HelloWorld;
impl Service for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        let mut core = Core::new().unwrap();
        let client = Client::new(&core.handle());

        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64))
                .with_body(PHRASE)
        ))
    }
}

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    header! { ( Authorization, "Authorization" ) => [String] }
    // let auth_header = format!("token {}", token);
    // req.headers_mut().set(Authorization(auth_header));

    let mut req = Request::new(Method::Get, "https://api.github.com/repos/rchaser53/vue-table-playground/commits".parse().unwrap());
    req.headers_mut().set(UserAgent::new("todo"));

    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    let work = client.request(req)
                    .and_then(|res| {
                        res.body().concat2().and_then(move |body| {
                            let v: Value = serde_json::from_slice(&body).unwrap();
                            println!("{:?}", v);
                            Ok(v)
                        })
                    });
    let res = core.run(work).unwrap();
    // let addr = "127.0.0.1:3000".parse().unwrap();
    // let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
    // server.run().unwrap();
}

// header! { ( Authorization, "Authorization" ) => [String] }
// let auth_header = format!("token {}", token);

                // res.body().for_each(|chunk| {
                //     io::stdout()
                //         .write_all(&chunk)
                //         .map_err(From::from)
                // })

                // Response::new()
                    // .with_header(ContentLength(PHRASE.len() as u64))
                    // .with_body(PHRASE)
                    // .with_header(ContentLength(res.body().len() as u64))
                    // .with_body(res.body());