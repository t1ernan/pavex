use std::path::PathBuf;

use pavex_builder::{constructor::Lifecycle, f, router::GET, Blueprint};
use pavex_runtime::hyper::Body;
use pavex_runtime::request::RequestHead;
use pavex_runtime::response::Response;

// The call graph looks like this:
//
// Request
//  / \
// B   |&
//  \  |
// handler
//
// `Request` cannot be borrowed by `handler` after it has been moved to construct `B`.
// Pavex should detect this and report an error since `Request` is a framework built-in type and
// it is not marked as `CloneIfNecessary`.

pub struct B;

pub fn b(_r: RequestHead) -> B {
    todo!()
}

pub fn handler(_r: &RequestHead, _b: B) -> Response {
    todo!()
}

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.constructor(f!(crate::b), Lifecycle::RequestScoped);
    bp.route(GET, "/home", f!(crate::handler));
    bp
}
