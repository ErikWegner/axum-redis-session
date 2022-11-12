use axum::{routing::get, Router};
use axum_database_sessions::{
    AxumRedisPool, AxumSession, AxumSessionConfig, AxumSessionLayer, AxumSessionStore,
};
use std::net::SocketAddr;

async fn app() -> Router {
    let client = redis::Client::open("redis://redis/").unwrap();
    //This Defaults as normal Cookies.
    //To enable Private cookies for integrity, and authenticity please check the next Example.
    let session_config = AxumSessionConfig::default().with_cookie_name("sid");
    let pool = AxumRedisPool::from(client);

    let session_store = AxumSessionStore::<AxumRedisPool>::new(Some(pool), session_config);

    //Create the Database table for storing our Session Data.
    session_store.initiate().await.unwrap();

    // build our application with some routes
    Router::new()
        .route("/in", get(start_session))
        .route("/out", get(end_session))
        .layer(AxumSessionLayer::new(session_store))
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

async fn start_session(session: AxumSession<AxumRedisPool>) -> String {
    let mut count: usize = session.get("count").await.unwrap_or(0);

    count += 1;
    session.set("count", count).await;

    count.to_string()
}

async fn end_session(session: AxumSession<AxumRedisPool>) -> String {
    let mut count: usize = session.get("count").await.unwrap_or(0);

    count += 1;
    session.set("count", count).await;

    count.to_string()
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use crate::app;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use itertools::Itertools;
    use tower::ServiceExt; // for `app.oneshot()`

    #[tokio::test]
    async fn it_sets_cookie_header() {
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
        assert_eq!(2, cookie_headers.len());
    }
}
