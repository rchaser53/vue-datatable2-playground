extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;

use std::io::{self, Write};
use futures::future::{Future};
use futures::stream::{Stream};
use hyper::Client;
use tokio_core::reactor::Core;

use hyper::header::ContentLength;
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
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    let uri = "https://api.github.com/repos/rchaser53/vue-datatable2-playground/commits"
                    .parse().unwrap();
    let work = client.get(uri);

    let res = core.run(work).unwrap();
    println!("{:?}", res.status().is_success());

    // let addr = "127.0.0.1:3000".parse().unwrap();
    // let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
    // server.run().unwrap();
}


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