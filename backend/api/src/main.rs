use axum::{
    Json, Router,
    body::Body,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
};

use api::{
    api::{check_health, delete_file, get_all, upload_file},
    models::FileObjectMetadata,
    utils::get_file_op,
};
use opendal::Operator;
use tracing::{debug, info};
use tracing_subscriber::FmtSubscriber;

#[derive(Clone)]
pub struct AppState {
    pub op: Operator,
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter("debug") // or from RUST_LOG
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing::debug!("creating app state");
    let state = AppState {
        op: get_file_op().unwrap(),
    };
    // build our application with a single route
    let app = Router::new()
        .route("/check-health", get(check_health_endpoint))
        .route("/files", get(get_all_endpoint))
        .route("/upload", post(upload_endpoint))
        .route("/delete/{key}", delete(delete_file_endpoint))
        .with_state(state);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("starting server on localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

pub async fn check_health_endpoint() -> &'static str {
    return check_health().await;
}

pub async fn delete_file_endpoint(State(state): State<AppState>, Path(key): Path<String>) -> Result<Response, Response> {
    info!("received delete request for {}", key);
    delete_file(&state.op, &key).await.map_err(|err| {
        tracing::error!("error deleting file: {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, err).into_response()
    })?;
    Ok(StatusCode::OK.into_response())
}

pub async fn upload_endpoint(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Response, Response> {
    info!("received upload request");
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST.into_response())?
    {
        if let Some("files") = field.name() {
            let file_name = field.file_name().unwrap_or("NO_NAME").to_owned();
            let content_type = field.content_type().unwrap_or("NO_NAME").to_owned();
            let file_data = field.bytes().await.unwrap();
            debug!(
                "uploading file: {}, content_type: {}, size: {} bytes",
                file_name,
                content_type,
                file_data.len()
            );

            upload_file(&state.op, &file_name, &content_type, file_data.to_vec())
                .await
                .map_err(|err| {
                    tracing::error!("error uploading file: {:?}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, err).into_response()
                })?;
        }
    }
    Ok(StatusCode::OK.into_response())
}

pub async fn get_all_endpoint(
    State(state): State<AppState>,
) -> Result<Json<Vec<FileObjectMetadata>>, Response> {
    info!("received get request for all files");
    let metas = get_all(state.op).await.map_err(|err| {
        tracing::error!("error reading file metadata: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, err).into_response()
    })?;

    info!("responding with {} files", metas.len());
    debug!("{:?}", metas);

    Ok(Json(metas))
}
