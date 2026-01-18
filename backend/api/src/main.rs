use axum::{
    Json, Router,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
};

use api::{
    api::{check_health, delete_file, get_all_files, upload_file, upload_media},
    models::FileObjectMetadata,
    utils::{get_file_op, get_media_op},
};
use opendal::Operator;
use tracing::{debug, info};
use tracing_subscriber::FmtSubscriber;

#[derive(Clone)]
pub struct AppState {
    /// file operator
    pub fop: Operator,
    
    /// media operator
    pub mop: Operator,
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter("debug") // or from RUST_LOG
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing::debug!("creating app state");
    let state = AppState {
        fop: get_file_op().unwrap(),
        mop: get_media_op().unwrap()
    };
    // build our application with a single route
    let app = Router::new()
        .route("/check-health", get(check_health_endpoint))

        // files
        .route("/files", get(get_all_files_endpoint))
        .route("/upload-file", post(upload_file_endpoint))
        .route("/delete-file/{key}", delete(delete_file_endpoint))

        // media
        .route("/media", get(get_all_medias_endpoint))
        .route("/upload-media", post(upload_media_endpoint))

        // state
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
    delete_file(&state.fop, &key).await.map_err(|err| {
        tracing::error!("error deleting file: {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, err).into_response()
    })?;
    Ok(StatusCode::OK.into_response())
}

pub async fn upload_file_endpoint(
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

            upload_file(&state.fop, &file_name, &content_type, file_data.to_vec())
                .await
                .map_err(|err| {
                    tracing::error!("error uploading file: {:?}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, err).into_response()
                })?;
        }
    }
    Ok(StatusCode::OK.into_response())
}

pub async fn get_all_files_endpoint(
    State(state): State<AppState>,
) -> Result<Json<Vec<FileObjectMetadata>>, Response> {
    info!("received get request for all files");
    let metas = get_all_files(&state.fop).await.map_err(|err| {
        tracing::error!("error reading file metadata: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, err).into_response()
    })?;

    info!("responding with {} files", metas.len());
    debug!("{:?}", metas);

    Ok(Json(metas))
}

pub async fn get_all_medias_endpoint(
    State(state): State<AppState>,
) -> Result<Json<Vec<FileObjectMetadata>>, Response> {
    info!("received get request for all medias");
    let metas = get_all_files(&state.mop).await.map_err(|err| {
        tracing::error!("error reading media metadata: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, err).into_response()
    })?;

    info!("responding with {} medias", metas.len());
    debug!("{:?}", metas);

    Ok(Json(metas))
}

pub async fn upload_media_endpoint(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Response, Response> {
    info!("received upload request");
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST.into_response())?
    {
        if let Some("medias") = field.name() {
            let file_name = field.file_name().unwrap_or("NO_NAME").to_owned();
            let content_type = field.content_type().unwrap_or("NO_NAME").to_owned();
            let file_data = field.bytes().await.unwrap();
            debug!(
                "uploading media: {}, content_type: {}, size: {} bytes",
                file_name,
                content_type,
                file_data.len()
            );

            upload_media(&state.mop, &file_name, &content_type, file_data.to_vec())
                .await
                .map_err(|err| {
                    tracing::error!("error uploading file: {:?}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, err).into_response()
                })?;
        }
    }
    Ok(StatusCode::OK.into_response())
}