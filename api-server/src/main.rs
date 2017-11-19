extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;
extern crate serde_json;

use futures::future::{Future};
use hyper::{Client, Method, StatusCode};
use tokio_core::reactor::Core;

use hyper::header::{Headers, UserAgent, ContentLength};
use hyper::server::{Http, Request, Response, Service};
use hyper_tls::HttpsConnector;

struct HelloWorld;
impl Service for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle).unwrap())
            .build(&handle);

        match (req.method(), req.path()) {
            (&Method::Get, "/*") => {
                response.set_body("nya-n");
            },
            (&Method::Get, "/git") => {
                let mut req = Request::new(Method::Get, "https://api.github.com/repos/rchaser53/vue-table-playground/commits".parse().unwrap());
                req.headers_mut().set(UserAgent::new("todo"));
                let work = client.request(req);
                let _res = core.run(work).unwrap();
                response.set_body(_res.body());
            },
            _ => {
                response.set_status(StatusCode::NotFound);
            },
        };
        Box::new(futures::future::ok(response))
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
    server.run().unwrap();
}

            // Response::new()
            //     // .with_header(ContentLength(res.body().len() as u64))
            //     .with_body(res.body())

    // header! { ( Authorization, "Authorization" ) => [String] }
    // let auth_header = format!("token {}", token);
    // req.headers_mut().set(Authorization(auth_header));