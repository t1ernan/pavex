//! Do NOT edit this code.
//! It was automatically generated by Pavex.
//! All manual edits will be lost next time the code is generated.
#[allow(unused_imports)]
use std as alloc;
struct ServerState {
    router: pavex_runtime::routing::Router<u32>,
    #[allow(dead_code)]
    application_state: ApplicationState,
}
pub struct ApplicationState {}
pub async fn build_application_state() -> crate::ApplicationState {
    crate::ApplicationState {}
}
pub async fn run(
    server_builder: pavex_runtime::hyper::server::Builder<
        pavex_runtime::hyper::server::conn::AddrIncoming,
    >,
    application_state: ApplicationState,
) -> Result<(), pavex_runtime::Error> {
    let server_state = std::sync::Arc::new(ServerState {
        router: build_router().map_err(pavex_runtime::Error::new)?,
        application_state,
    });
    let make_service = pavex_runtime::hyper::service::make_service_fn(move |_| {
        let server_state = server_state.clone();
        async move {
            Ok::<
                _,
                pavex_runtime::hyper::Error,
            >(
                pavex_runtime::hyper::service::service_fn(move |request| {
                    let server_state = server_state.clone();
                    async move {
                        Ok::<
                            _,
                            pavex_runtime::hyper::Error,
                        >(route_request(request, server_state).await)
                    }
                }),
            )
        }
    });
    server_builder.serve(make_service).await.map_err(pavex_runtime::Error::new)
}
fn build_router() -> Result<
    pavex_runtime::routing::Router<u32>,
    pavex_runtime::routing::InsertError,
> {
    let mut router = pavex_runtime::routing::Router::new();
    router.insert("/home/:home_id", 0u32)?;
    router.insert("/home/:home_id/room/:room_id", 1u32)?;
    router.insert("/town/*town", 2u32)?;
    Ok(router)
}
async fn route_request(
    request: http::Request<pavex_runtime::hyper::body::Body>,
    server_state: std::sync::Arc<ServerState>,
) -> pavex_runtime::response::Response {
    #[allow(unused)]
    let (request_head, request_body) = request.into_parts();
    let request_head: pavex_runtime::request::RequestHead = request_head.into();
    let matched_route = match server_state.router.at(&request_head.uri.path()) {
        Ok(m) => m,
        Err(_) => {
            return pavex_runtime::response::Response::builder()
                .status(pavex_runtime::http::StatusCode::NOT_FOUND)
                .body(pavex_runtime::body::boxed(hyper::body::Body::empty()))
                .unwrap();
        }
    };
    let route_id = matched_route.value;
    #[allow(unused)]
    let url_params: pavex_runtime::extract::route::RawRouteParams<'_, '_> = matched_route
        .params
        .into();
    match route_id {
        0u32 => {
            match &request_head.method {
                &pavex_runtime::http::Method::GET => route_handler_0(url_params).await,
                _ => {
                    pavex_runtime::response::Response::builder()
                        .status(pavex_runtime::http::StatusCode::METHOD_NOT_ALLOWED)
                        .header(pavex_runtime::http::header::ALLOW, "GET")
                        .body(pavex_runtime::body::boxed(hyper::body::Body::empty()))
                        .unwrap()
                }
            }
        }
        1u32 => {
            match &request_head.method {
                &pavex_runtime::http::Method::GET => route_handler_1(url_params).await,
                _ => {
                    pavex_runtime::response::Response::builder()
                        .status(pavex_runtime::http::StatusCode::METHOD_NOT_ALLOWED)
                        .header(pavex_runtime::http::header::ALLOW, "GET")
                        .body(pavex_runtime::body::boxed(hyper::body::Body::empty()))
                        .unwrap()
                }
            }
        }
        2u32 => {
            match &request_head.method {
                &pavex_runtime::http::Method::GET => route_handler_2(url_params).await,
                _ => {
                    pavex_runtime::response::Response::builder()
                        .status(pavex_runtime::http::StatusCode::METHOD_NOT_ALLOWED)
                        .header(pavex_runtime::http::header::ALLOW, "GET")
                        .body(pavex_runtime::body::boxed(hyper::body::Body::empty()))
                        .unwrap()
                }
            }
        }
        _ => {
            pavex_runtime::response::Response::builder()
                .status(pavex_runtime::http::StatusCode::NOT_FOUND)
                .body(pavex_runtime::body::boxed(hyper::body::Body::empty()))
                .unwrap()
        }
    }
}
pub async fn route_handler_0(
    v0: pavex_runtime::extract::route::RawRouteParams<'_, '_>,
) -> http::Response<
    http_body::combinators::BoxBody<bytes::Bytes, pavex_runtime::Error>,
> {
    let v1 = pavex_runtime::extract::route::RouteParams::extract(v0);
    match v1 {
        Ok(v2) => {
            let v3 = app::get_home(v2);
            <alloc::string::String as pavex_runtime::response::IntoResponse>::into_response(
                v3,
            )
        }
        Err(v2) => {
            let v3 = pavex_runtime::extract::route::errors::ExtractRouteParamsError::into_response(
                &v2,
            );
            <http::Response<
                alloc::string::String,
            > as pavex_runtime::response::IntoResponse>::into_response(v3)
        }
    }
}
pub async fn route_handler_1(
    v0: pavex_runtime::extract::route::RawRouteParams<'_, '_>,
) -> http::Response<
    http_body::combinators::BoxBody<bytes::Bytes, pavex_runtime::Error>,
> {
    let v1 = pavex_runtime::extract::route::RouteParams::extract(v0);
    match v1 {
        Ok(v2) => {
            let v3 = app::get_room(v2);
            <alloc::string::String as pavex_runtime::response::IntoResponse>::into_response(
                v3,
            )
        }
        Err(v2) => {
            let v3 = pavex_runtime::extract::route::errors::ExtractRouteParamsError::into_response(
                &v2,
            );
            <http::Response<
                alloc::string::String,
            > as pavex_runtime::response::IntoResponse>::into_response(v3)
        }
    }
}
pub async fn route_handler_2(
    v0: pavex_runtime::extract::route::RawRouteParams<'_, '_>,
) -> http::Response<
    http_body::combinators::BoxBody<bytes::Bytes, pavex_runtime::Error>,
> {
    let v1 = pavex_runtime::extract::route::RouteParams::extract(v0);
    match v1 {
        Ok(v2) => {
            let v3 = app::get_town(v2);
            <alloc::string::String as pavex_runtime::response::IntoResponse>::into_response(
                v3,
            )
        }
        Err(v2) => {
            let v3 = pavex_runtime::extract::route::errors::ExtractRouteParamsError::into_response(
                &v2,
            );
            <http::Response<
                alloc::string::String,
            > as pavex_runtime::response::IntoResponse>::into_response(v3)
        }
    }
}