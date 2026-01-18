/// api endpoints
use opendal::{Operator, options::WriteOptions};
use tracing::{debug, info};

use crate::models::FileObjectMetadata;

pub async fn check_health() -> &'static str {
    "Everything looks good."
}

pub async fn delete_file(op: &Operator, name: &str) -> Result<(), String> {
    op.delete(name).await.map_err(|err| format!("failed to delete file: {}", err))?;
    info!("successfully deleted file: {}", name);
    Ok(())
}

pub async fn upload_file(op: &Operator, name: &str, ftype: &str, bytes: Vec<u8>) -> Result<FileObjectMetadata, String> {
    let write_options = WriteOptions {
        content_type: Some(ftype.to_string()),
        ..Default::default()
    };
    debug!("create write options: {:?} for {}", write_options, name );
    let raw_metadata = op.write_options(name, bytes, write_options).await.map_err(|err| format!("failed to write data: {}", err))?;
    debug!("received raw metadata: {:?} for {}", raw_metadata, name);
    let metadata = FileObjectMetadata::try_from((name.to_string(), raw_metadata)).map_err(|_| format!("unable to read metadata"))?;
    info!("successfully uploaded file: {}", name);
    debug!("file metadata: {:?}", metadata);
    Ok(metadata)
}

pub async fn upload_media(op: &Operator, name: &str, ftype: &str, bytes: Vec<u8>) -> Result<FileObjectMetadata, String> {
    let write_options = WriteOptions {
        content_type: Some(ftype.to_string()),
        ..Default::default()
    };
    debug!("create write options: {:?} for {}", write_options, name );
    let raw_metadata = op.write_options(name, bytes, write_options).await.map_err(|err| format!("failed to write data: {}", err))?;
    debug!("received raw metadata: {:?} for {}", raw_metadata, name);
    let metadata = FileObjectMetadata::try_from((name.to_string(), raw_metadata)).map_err(|_| format!("unable to read metadata"))?;
    info!("successfully uploaded file: {}", name);
    debug!("file metadata: {:?}", metadata);
    Ok(metadata)
}

pub async fn get_all_files(op: &Operator) -> Result<Vec<FileObjectMetadata>, String> {
  let files = op.list("/").await.map_err(|err| format!("unable to read files: {}", err))?;
  debug!("read files from storage: {:?}", files);

  debug!("transforming into server model type");
  let metas = files
    .into_iter()
    .map(|file| FileObjectMetadata::try_from(file).map_err(|err| format!("unable to get metadata from files: {}", err)))
    .collect::<Result<Vec<FileObjectMetadata>, String>>()?;

  Ok(metas)
}

pub async fn get_all_medias(op: &Operator) -> Result<Vec<FileObjectMetadata>, String> {
  let files = op.list("/").await.map_err(|err| format!("unable to read medias: {}", err))?;
  debug!("read medias from storage: {:?}", files);

  debug!("transforming into server model type");
  let metas = files
    .into_iter()
    .map(|file| FileObjectMetadata::try_from(file).map_err(|err| format!("unable to get metadata from medias: {}", err)))
    .collect::<Result<Vec<FileObjectMetadata>, String>>()?;

  Ok(metas)
}

