use pavex::http::StatusCode;
use pavex::request::path::RawPathParams;

pub fn handler(params: &RawPathParams) -> StatusCode {
    for (name, value) in params.iter() {
        println!("`{name}` was set to `{}`", value.as_str());
    }
    StatusCode::OK
}
