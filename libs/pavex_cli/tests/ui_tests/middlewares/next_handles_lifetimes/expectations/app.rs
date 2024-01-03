//! Do NOT edit this code.
//! It was automatically generated by Pavex.
//! All manual edits will be lost next time the code is generated.
extern crate alloc;
struct ServerState {
    router: matchit::Router<u32>,
    #[allow(dead_code)]
    application_state: ApplicationState,
}
pub struct ApplicationState {}
pub async fn build_application_state() -> crate::ApplicationState {
    crate::ApplicationState {}
}
pub fn run(
    server_builder: pavex::server::Server,
    application_state: ApplicationState,
) -> pavex::server::ServerHandle {
    let server_state = std::sync::Arc::new(ServerState {
        router: build_router(),
        application_state,
    });
    server_builder.serve(route_request, server_state)
}
fn build_router() -> matchit::Router<u32> {
    let mut router = matchit::Router::new();
    router.insert("/home", 0u32).unwrap();
    router
}
async fn route_request(
    request: http::Request<hyper::body::Incoming>,
    server_state: std::sync::Arc<ServerState>,
) -> pavex::response::Response {
    let (request_head, request_body) = request.into_parts();
    #[allow(unused)]
    let request_body = pavex::request::body::RawIncomingBody::from(request_body);
    let request_head: pavex::request::RequestHead = request_head.into();
    let matched_route = match server_state.router.at(&request_head.target.path()) {
        Ok(m) => m,
        Err(_) => {
            let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter(
                    vec![],
                )
                .into();
            return route_1::middleware_0(&allowed_methods).await;
        }
    };
    let route_id = matched_route.value;
    #[allow(unused)]
    let url_params: pavex::request::path::RawPathParams<'_, '_> = matched_route
        .params
        .into();
    match route_id {
        0u32 => {
            match &request_head.method {
                &pavex::http::Method::GET => route_0::middleware_0().await,
                _ => {
                    let allowed_methods: pavex::router::AllowedMethods = pavex::router::MethodAllowList::from_iter([
                            pavex::http::Method::GET,
                        ])
                        .into();
                    route_1::middleware_0(&allowed_methods).await
                }
            }
        }
        i => unreachable!("Unknown route id: {}", i),
    }
}
pub mod route_0 {
    pub async fn middleware_0() -> pavex::response::Response {
        let v0 = app::c();
        let v1 = app::a();
        let v2 = app::b(&v1, &v0);
        let v3 = crate::route_0::Next0 {
            s_0: &v1,
            s_1: &v0,
            next: handler,
        };
        let v4 = pavex::middleware::Next::new(v3);
        app::mw(v4, v2)
    }
    pub async fn handler(v0: &app::A, v1: &app::C) -> pavex::response::Response {
        let v2 = app::handler(v0, v1);
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v2)
    }
    pub struct Next0<'a, 'b, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a app::A,
        s_1: &'b app::C,
        next: fn(&'a app::A, &'b app::C) -> T,
    }
    impl<'a, 'b, T> std::future::IntoFuture for Next0<'a, 'b, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0, self.s_1)
        }
    }
}
pub mod route_1 {
    pub async fn middleware_0(
        v0: &pavex::router::AllowedMethods,
    ) -> pavex::response::Response {
        let v1 = app::c();
        let v2 = app::a();
        let v3 = app::b(&v2, &v1);
        let v4 = crate::route_1::Next0 {
            s_0: v0,
            next: handler,
        };
        let v5 = pavex::middleware::Next::new(v4);
        app::mw(v5, v3)
    }
    pub async fn handler(
        v0: &pavex::router::AllowedMethods,
    ) -> pavex::response::Response {
        let v1 = pavex::router::default_fallback(v0).await;
        <pavex::response::Response as pavex::response::IntoResponse>::into_response(v1)
    }
    pub struct Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        s_0: &'a pavex::router::AllowedMethods,
        next: fn(&'a pavex::router::AllowedMethods) -> T,
    }
    impl<'a, T> std::future::IntoFuture for Next0<'a, T>
    where
        T: std::future::Future<Output = pavex::response::Response>,
    {
        type Output = pavex::response::Response;
        type IntoFuture = T;
        fn into_future(self) -> Self::IntoFuture {
            (self.next)(self.s_0)
        }
    }
}