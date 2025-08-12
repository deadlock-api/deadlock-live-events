use core::time::Duration;

use async_stream::try_stream;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use bytes::Bytes;
use futures::Stream;
use haste::broadcast::{BroadcastHttp, BroadcastHttpClientError};
use tracing::info;

use crate::error::{APIError, APIResult};
use crate::state::AppState;
use crate::utils::{live_demo_exists, spectate_match};

fn demo_stream(
    match_id: u64,
) -> impl Stream<Item = Result<Bytes, BroadcastHttpClientError<reqwest::Error>>> {
    let client = reqwest::Client::new();
    try_stream! {
        let mut demofile = BroadcastHttp::start_streaming(
            client,
            format!("https://dist1-ord1.steamcontent.com/tv/{match_id}"),
        ).await?;
        while let Some(chunk) = demofile.next_packet().await {
            info!("Received chunk");
            yield chunk?;
        }
    }
}

pub(super) async fn demo(
    Path(match_id): Path<u64>,
    State(state): State<AppState>,
) -> APIResult<impl IntoResponse> {
    info!("Spectating match {match_id}");
    tryhard::retry_fn(|| {
        spectate_match(
            &state.http_client,
            match_id,
            state.config.deadlock_api_key.as_ref().map(AsRef::as_ref),
        )
    })
    .retries(3)
    .fixed_backoff(Duration::from_millis(200))
    .await?;

    // Wait for the demo to be available
    tryhard::retry_fn(|| async {
        live_demo_exists(&state.http_client, match_id)
            .await
            .then_some(())
            .ok_or(())
    })
    .retries(60)
    .fixed_backoff(Duration::from_millis(500))
    .await
    .map_err(|()| APIError::internal("Failed to spectate match"))?;

    Ok(Body::from_stream(demo_stream(match_id)))
}
