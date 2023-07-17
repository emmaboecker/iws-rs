use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect, Response},
};
use mongodb::{bson::doc, options::FindOneAndReplaceOptions};
use rand::{distributions::Alphanumeric, Rng};
use reqwest::{header, StatusCode};
use url::Url;

use crate::database::VerifiedGuild;

use self::discord_util::generate_discord_path;

use super::WebServerState;

mod discord_util;

pub async fn invite_accept_url(
    State(state): State<Arc<WebServerState>>,
    Path(invite_id): Path<String>,
) -> Response {
    let invite = state
        .collections
        .invites
        .find_one_and_delete(doc! { "_id": invite_id }, None)
        .await
        .unwrap();

    if invite.is_none() {
        return (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/plain")],
            "Diese Einladung ist nicht (mehr) gültig!",
        )
            .into_response();
    }

    let invite = invite.unwrap();

    let nonce: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();

    state
        .states
        .lock()
        .unwrap()
        .insert(nonce.clone(), invite.clone());

    Redirect::to(&generate_discord_path(invite.guild_id, nonce)).into_response()
}

#[derive(serde::Deserialize)]
pub struct DoneRouteQuery {
    pub code: String,
    pub state: String,
}

pub async fn done_route(
    State(state): State<Arc<WebServerState>>,
    Query(query_params): Query<DoneRouteQuery>,
) -> Response {
    let client_id = std::env::var("DISCORD_CLIENT_ID").unwrap();
    let client_id = client_id.as_str();
    let client_secret = std::env::var("DISCORD_CLIENT_SECRET").unwrap();
    let client_secret = client_secret.as_str();

    let redirect_url =
        Url::parse(&format!("{}/done", std::env::var("WEBSERVER_URL").unwrap())).unwrap();
    let redirect_url = redirect_url.as_str();

    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("code", &query_params.code),
        ("grant_type", "authorization_code"),
        ("redirect_uri", redirect_url),
    ];

    let res = state
        .http_client
        .post("https://discord.com/api/v10/oauth2/token")
        .form(&params)
        .send()
        .await;

    if res.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(header::CONTENT_TYPE, "text/plain")],
            "Ein Fehler ist bei der Verbindung mit Discord aufgetreten! Bitte versuche es später erneut. Wenn dieser Fehler weiterhin auftritt, melde dich bitte bei der Entwickler:in.",
        )
            .into_response();
    }

    let res = res.unwrap();

    if !res.status().is_success() {
        return (
            StatusCode::FORBIDDEN,
            [(header::CONTENT_TYPE, "text/plain")],
            format!(
                "Die Authentifizierung mit Discord hat nicht geklappt: {}",
                res.text().await.unwrap()
            ),
        )
            .into_response();
    }

    let invite = state.states.lock().unwrap().remove(&query_params.state);

    if invite.is_none() {
        return (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/plain")],
            "Diese Einladung ist nicht (mehr) gültig! Wie hast du das geschafft?",
        )
            .into_response();
    }

    let invite = invite.unwrap();

    state
        .collections
        .verified_guilds
        .find_one_and_replace(
            doc! { "_id": invite.guild_id.to_string() },
            VerifiedGuild {
                guild_id: invite.guild_id,
                verified: true,
            },
            FindOneAndReplaceOptions::builder().upsert(true).build(),
        )
        .await
        .unwrap();

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain")],
        "IWS ist nun auf deinem Server! ",
    )
        .into_response()
}
