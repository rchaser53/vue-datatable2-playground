extern crate hyper;
extern crate futures;
extern crate tokio_core;

use hyper::Client;
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};

use futures::future::Future;

use tokio_core::reactor::Core;

const PHRASE: &'static str = "Hello, World!";

struct HelloWorld;
impl Service for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        // We're currently ignoring the Request
        // And returning an 'ok' Future, which means it's ready
        // immediately, and build a Response with the 'PHRASE' body.
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
    // let mut core = Core::new().unwrap();
    // let client = Client::new(&core.handle());

    // let uri = "https://api.github.com/repos/rchaser53/vue-datatable2-playground/commits".parse().unwrap();

    // let res = client.get(uri)
    //             .map(|res| {
                // println!("{}", res);
                // println!("Response: {}", res.status());
                // println!(11);

                // Response::new()
                    // .with_header(ContentLength(PHRASE.len() as u64))
                    // .with_body(PHRASE)
                    // .with_header(ContentLength(res.body().len() as u64))
                    // .with_body(res.body());
            // });


    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
    server.run().unwrap();
}