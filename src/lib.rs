extern crate bytes;
extern crate futures;
extern crate httparse;
extern crate net2;
extern crate num_cpus;
extern crate regex;
extern crate serde;
extern crate time;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

#[macro_use] extern crate templatify;
// For tests
#[allow(unused_imports)]
#[macro_use] extern crate serde_derive;
#[allow(unused_imports)]
#[macro_use] extern crate serde_json;

mod app;
mod builtins;
mod context;
mod date;
mod middleware;
mod request;
mod response;
mod route_parser;
mod http;
mod util;

pub use app::{App, AppService};
pub use builtins::send::file;
pub use context::{BasicContext, Context};
pub use middleware::{Middleware, MiddlewareChain, MiddlewareReturnValue};
pub use request::Request;
pub use response::Response;
pub use http::Http;
