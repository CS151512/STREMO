use axum::{extract::{
    State, Query
}, Json};
use std::sync::Arc;


use crate::{
    models::{
        requests ::GetStreamsQeury,
        responses::{StreamCatalogItem, StreamCatalogResponse}
    }
    utils::errors::AppError,
    AppState,
};


pub async fn get_live_streams_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<GetStreamQuery>,
) -> Result<Json<StreamCatalogResponse>, AppError> {
    let limit  = query.limit.unwrap_or(20);
    let cursor = query.cursor.unwrap_or_default();

    let mut stream_client =
        state.stream_meta_grpc_client.clone();

    let grpc_res =
        stream_client.get_live_streams(limit,
            cursor).await?;

    let streams = grpc_res.streams.into_iter().map(|s| StreamCatalogItem {
        stream_id: s.stream_id,
        title: s.title,
        category: s.category,
        viewers_count: s.viewers_count,
    }).collect();

    Ok(Json(StreamCatalogResponse {
        data: streams,
        next_cursor: grpc_res.next_cursor,
    }))
}
