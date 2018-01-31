extern crate futures;
extern crate hyper;
extern crate tokio_core;


use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());

    let uri = "http://httpbin.org/ip".parse()?;
    let work = client.get(uri).and_then(|res| {
        println!("res: {}", res.status);
        res.body().for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map_err(From::from)
        })
    });
    core.run(work)?;
}