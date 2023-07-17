use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{routing::get, Router};

use crate::database::{IWSCollections, Invitation};

mod verification;
use verification::*;

#[derive(Clone)]
pub struct WebServerState {
    pub collections: Arc<IWSCollections>,
    pub states: Arc<Mutex<HashMap<String, Invitation>>>,
    pub http_client: Arc<reqwest::Client>,
}

pub async fn http_server(collections: Arc<IWSCollections>) -> eyre::Result<()> {
    let listen_url = std::env::var("HTTP_LISTEN_URL")?;

    let state = Arc::new(WebServerState {
        collections,
        states: Arc::new(Mutex::new(HashMap::new())),
        http_client: Arc::new(reqwest::Client::new()),
    });

    let app = Router::new()
        .route("/invitation/accept/:invite_id", get(invite_accept_url))
        .route("/done", get(done_route))
        .with_state(state);

    axum::Server::bind(&listen_url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
