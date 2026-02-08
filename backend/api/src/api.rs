use std::collections::HashSet;
use std::sync::Arc;

use crate::models::MediaOperators;
use crate::{constants::DEFAULT_THUMBNAIL_SIZE, models::MediaOperatorsBuf};
use futures::{Stream, StreamExt};
/// api endpoints
use opendal::{Operator, options::WriteOptions};
use tokio::task::JoinSet;
use tracing::{debug, info};

use crate::{models::FileObjectMetadata, processing::generate_thumbnail};

pub async fn check_health() -> &'static str {
    "Everything looks good."
}

pub async fn delete_file(op: &Operator, name: &str) -> Result<(), String> {
    op.delete(name)
        .await
        .map_err(|err| format!("failed to delete file: {}", err))?;
    info!("successfully deleted file: {}", name);
    Ok(())
}

pub async fn upload_file(
    op: &Operator,
    name: &str,
    ftype: &str,
    bytes: Vec<u8>,
) -> Result<FileObjectMetadata, String> {
    let write_options = WriteOptions {
        content_type: Some(ftype.to_string()),
        ..Default::default()
    };
    debug!("create write options: {:?} for {}", write_options, name);
    let raw_metadata = op
        .write_options(name, bytes, write_options)
        .await
        .map_err(|err| format!("failed to write data: {}", err))?;
    debug!("received raw metadata: {:?} for {}", raw_metadata, name);
    let metadata = FileObjectMetadata::try_from((name.to_string(), raw_metadata))
        .map_err(|_| format!("unable to read metadata"))?;
    info!("successfully uploaded file: {}", name);
    debug!("file metadata: {:?}", metadata);
    Ok(metadata)
}

pub async fn upload_chunk_stream<'a, F: StreamExt<Item = Vec<u8>> + Unpin>(
    op: &Operator,
    key: &str,
    ftype: &str,
    mut chunks: F,
) -> Result<FileObjectMetadata, String> {
    // let w = op.writer(key)
    let write_options = WriteOptions {
        content_type: Some(ftype.to_string()),
        ..Default::default()
    };
    debug!("create write options: {:?} for {}", write_options, key);

    let mut w = op.writer_options(key, write_options).await.unwrap();
    while let Some(chunk) = chunks.next().await {
        println!("got chunk of size: {} for file: {}", chunk.len(), key);
        _ = w.write(chunk.to_owned()).await;
    }
    let raw_metadata = w
        .close()
        .await
        .expect("could not close the write and get metadata");
    debug!("received raw metadata: {:?} for {}", raw_metadata, key);

    let metadata = FileObjectMetadata::try_from((key.to_string(), raw_metadata))
        .map_err(|_| format!("unable to read metadata"))?;
    info!("successfully uploaded file: {}", key);
    debug!("file metadata: {:?}", metadata);
    Ok(metadata)
}

pub async fn upload_media(
    mops: MediaOperators<'_>,
    name: &str,
    ftype: &str,
    bytes: Vec<u8>,
) -> Result<FileObjectMetadata, String> {
    let thumbnail = generate_thumbnail(&bytes, DEFAULT_THUMBNAIL_SIZE.into())
        .map_err(|err| format!("failed to generate thumbnail: {:?}", err))?;

    let write_options = WriteOptions {
        content_type: Some(ftype.to_string()),
        ..Default::default()
    };
    debug!("create write options: {:?} for {}", write_options, name);

    let (img_metadata, thumbnail_metadata) = tokio::join!(
        mops.mop.write_options(name, bytes, write_options.clone()),
        mops.top.write_options(name, thumbnail, write_options)
    );

    let raw_metadata = img_metadata.map_err(|err| format!("failed to write data: {}", err))?;
    debug!("received raw metadata: {:?} for {}", raw_metadata, name);

    let metadata = FileObjectMetadata::try_from((name.to_string(), raw_metadata))
        .map_err(|_| format!("unable to read metadata"))?;
    info!("successfully uploaded file: {}", name);
    debug!("file metadata: {:?}", metadata);
    Ok(metadata)
}

pub async fn get_all_files(op: &Operator) -> Result<Vec<FileObjectMetadata>, String> {
    let files = op
        .list("/")
        .await
        .map_err(|err| format!("unable to read files: {}", err))?;
    debug!("read files from storage: {:?}", files);

    debug!("transforming into server model type");
    let metas = files
        .into_iter()
        .map(|file| {
            FileObjectMetadata::try_from(file)
                .map_err(|err| format!("unable to get metadata from files: {}", err))
        })
        .collect::<Result<Vec<FileObjectMetadata>, String>>()?;

    Ok(metas)
}

pub async fn get_all_medias(op: &Operator) -> Result<Vec<FileObjectMetadata>, String> {
    let files = op
        .list("/")
        .await
        .map_err(|err| format!("unable to read medias: {}", err))?;
    debug!("read medias from storage: {:?}", files);

    debug!("transforming into server model type");
    let metas = files
        .into_iter()
        .map(|file| {
            FileObjectMetadata::try_from(file)
                .map_err(|err| format!("unable to get metadata from medias: {}", err))
        })
        .collect::<Result<Vec<FileObjectMetadata>, String>>()?;

    Ok(metas)
}

/// reads bytes from object store
pub async fn get_media(op: &Operator, key: &str) -> Result<Vec<u8>, String> {
    let bytes: Vec<u8> = op
        .read(key)
        .await
        .map_err(|err| format!("unable to read file from storage: {:?}", err))?
        .to_vec();
    Ok(bytes)
}

pub async fn task_generate_thumbnails_run(ops: MediaOperatorsBuf) {
    let (medias, thumbnails) = tokio::join!(get_all_medias(&ops.mop), get_all_medias(&ops.top));

    let keys_of_thumbnails: HashSet<_> = (&thumbnails)
        .as_deref()
        .unwrap()
        .iter()
        .map(|t| t.key.as_str())
        .collect();
    let keys_of_media_wo_thumbnails: Vec<String> = (&medias)
        .as_deref()
        .unwrap()
        .iter()
        .filter_map(|m| {
            if keys_of_thumbnails.contains(m.key.as_str()) {
                None
                // Some(m.key.to_owned())
            } else {
                Some(m.key.to_owned())
            }
        })
        .collect();

    let mop = Arc::new(ops.mop);
    let top = Arc::new(ops.top);

    // concurrent implementation gives 850 % improvement
    for key in keys_of_media_wo_thumbnails {
        let mop = mop.clone();
        let top = top.clone();
        tokio::spawn(async move {
            println!("spawned task for: {}", &key);
            let data = get_media(&mop, &key).await.unwrap();
            let thumbnail = tokio::task::spawn_blocking(move || {
                generate_thumbnail(&data, DEFAULT_THUMBNAIL_SIZE.into()).unwrap()
            }).await.unwrap();
            top.write(&key, thumbnail).await.unwrap();
            println!("finished task for: {}", &key);
        });
    }
}
