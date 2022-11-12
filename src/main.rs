use axum::{routing::get, Router};
use axum_database_sessions::{
    AxumRedisPool, AxumSession, AxumSessionConfig, AxumSessionLayer, AxumSessionStore,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let client = redis::Client::open("redis://redis/").unwrap();
    //This Defaults as normal Cookies.
    //To enable Private cookies for integrity, and authenticity please check the next Example.
    let session_config = AxumSessionConfig::default();
    let pool = AxumRedisPool::from(client);

    let session_store = AxumSessionStore::<AxumRedisPool>::new(Some(pool), session_config);

    //Create the Database table for storing our Session Data.
    session_store.initiate().await.unwrap();

    // build our application with some routes
    let app = Router::new()
        .route("/in", get(start_session))
        .route("/out", get(end_session))
        .layer(AxumSessionLayer::new(session_store));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
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
