#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::{env, io};

use actix_web::http::{StatusCode};
use actix_web::{
    middleware, App, HttpResponse, HttpServer, HttpRequest,
    Result,
};
use bytes::Bytes;

#[get("/hc")]
fn hc(req: HttpRequest) ->  Result<HttpResponse> {
   info!("{:?}", req);
   Ok(HttpResponse::build(StatusCode::OK)
        .content_type("plain/text; charset=utf-8")
        .body(Bytes::from(&b"OK"[..])))
}

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let sys = actix_rt::System::new("basic-example");

    HttpServer::new(|| {
        App::new()
            // enable logger
            //.wrap(middleware::Logger::default())
            .service(hc)
    })
    .bind("0.0.0.0:8787")?
    .start();

    sys.run()
}