use async_redis_session::RedisSessionStore;
use axum::{routing::get, Router};
use axum_sessions::{extractors::WritableSession, SessionLayer};
use std::net::SocketAddr;

async fn app() -> Router {
    let store = RedisSessionStore::new("redis://redis/").unwrap();
    let secret =
    String::from("f6b41bf46eda6d0606ab7c1ceb1b7c97ad9e347f2dd4cfeb323938e1abeb4c65a98f18952e4718450478ca908916db5ba6bed92af7b8dd56d6fe3b7c2823f565");
    let session_layer = SessionLayer::new(store, secret.as_bytes())
        .with_cookie_name("sid")
        .with_save_unchanged(false);

    // build our application with some routes
    Router::new()
        .route("/in", get(start_session))
        .route("/out", get(end_session))
        .route("/other", get(other))
        .layer(session_layer)
}

#[tokio::main]
async fn main() {
    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app().await.into_make_service())
        .await
        .unwrap();
}

async fn other() -> String {
    "No session required".to_owned()
}

async fn start_session(mut session: WritableSession) -> String {
    let mut count: usize = session.get("count").unwrap_or(0);

    count += 1;
    let _ = session.insert("count", count);

    count.to_string()
}

async fn end_session(mut session: WritableSession) -> String {
    let count: usize = session.get("count").unwrap_or(0);

    session.destroy();
    format!("Bye {}", count)
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use crate::app;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http::header::{COOKIE, SET_COOKIE};
    use itertools::Itertools;
    use tower::ServiceExt; // for `app.oneshot()`

    #[tokio::test]
    async fn it_sets_cookie_header_for_route_when_not_present() {
        let app = app().await;

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(Request::builder().uri("/in").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let cookie_headers = response
            .headers()
            .get_all(http::header::SET_COOKIE)
            .iter()
            .map(|a| from_utf8(a.as_bytes()).unwrap())
            .sorted();
        assert_eq!(1, cookie_headers.len());
    }

    #[tokio::test]
    async fn it_does_not_set_cookie_header_for_route_when_no_session_is_required() {
        let app1 = app().await;

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response1 = app1
            .oneshot(Request::builder().uri("/in").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let app2 = app().await;
        let response = app2
            .oneshot(
                Request::builder()
                    .uri("/in")
                    .header(COOKIE, response1.headers().get(SET_COOKIE).unwrap())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let cookie_headers = response
            .headers()
            .get_all(http::header::SET_COOKIE)
            .iter()
            .map(|a| from_utf8(a.as_bytes()).unwrap())
            .sorted();
        assert_eq!(0, cookie_headers.len());
    }

    #[tokio::test]
    async fn it_does_not_set_cookie_header_for_route_when_already_present() {
        let app = app().await;

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/other")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let cookie_headers = response
            .headers()
            .get_all(http::header::SET_COOKIE)
            .iter()
            .map(|a| from_utf8(a.as_bytes()).unwrap())
            .sorted();
        assert_eq!(0, cookie_headers.len());
    }
}
