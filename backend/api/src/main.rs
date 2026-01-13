use axum::{
    Json, Router,
    extract::{Multipart, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    body::Body
};

use api::{
    api::{check_health, get_all, upload_file},
    models::FileObjectMetadata,
    utils::get_file_op,
};
use opendal::Operator;

#[derive(Clone)]
pub struct AppState {
    pub op: Operator,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        op: get_file_op().unwrap(),
    };
    // build our application with a single route
    let app = Router::new()
        .route("/check-health", get(check_health_wrapper))
        .route("/files", get(get_all_wrapper))
        .route("/upload", post(upload_wrapper))
        .with_state(state);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn check_health_wrapper() -> &'static str {
    return check_health().await;
}

pub async fn upload_wrapper(mut multipart: Multipart) -> Result<Response, Response> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST.into_response())?
    {
        if let Some("files") = field.name() {
            upload_file(
                field.file_name().unwrap_or("_"),
                field.content_type().unwrap_or("_"),
            )
            .await?;
        }
    }
    Ok(StatusCode::OK.into_response())
}

pub async fn get_all_wrapper(
    State(state): State<AppState>,
) -> Result<Json<Vec<FileObjectMetadata>>, Response> {
    let metas = get_all(state.op)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err).into_response())?;
    Ok(Json(metas))
}
