//! Do NOT edit this code.
//! It was automatically generated by Pavex.
//! All manual edits will be lost next time the code is generated.
#[allow(unused_imports)]
use std as alloc;
struct ServerState {
    router: pavex_runtime::routing::Router<u32>,
    application_state: ApplicationState,
}
pub struct ApplicationState {
    s0: app::LoggerFactory,
}
pub async fn build_application_state() -> crate::ApplicationState {
    let v0 = app::LoggerFactory::new();
    crate::ApplicationState { s0: v0 }
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
                        let response = route_request(request, server_state).await;
                        let response = pavex_runtime::hyper::Response::from(response);
                        Ok::<_, pavex_runtime::hyper::Error>(response)
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
    router.insert("/home", 0u32)?;
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
            return pavex_runtime::response::Response::not_found().box_body();
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
                &pavex_runtime::http::Method::GET => {
                    route_handler_0(&server_state.application_state.s0).await
                }
                _ => {
                    let header_value = pavex_runtime::http::HeaderValue::from_static(
                        "GET",
                    );
                    pavex_runtime::response::Response::method_not_allowed()
                        .insert_header(pavex_runtime::http::header::ALLOW, header_value)
                        .box_body()
                }
            }
        }
        _ => pavex_runtime::response::Response::not_found().box_body(),
    }
}
pub async fn route_handler_0(
    v0: &app::LoggerFactory,
) -> pavex_runtime::response::Response {
    let v1 = app::Streamer::new();
    let v3 = {
        let v2 = app::LoggerFactory::logger(v0);
        app::Streamer::stream_file(&v1, v2)
    };
    <pavex_runtime::response::Response as pavex_runtime::response::IntoResponse>::into_response(
        v3,
    )
}