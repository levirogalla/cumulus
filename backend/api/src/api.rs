/// api endpoints

use axum::{
   http::StatusCode, response::{IntoResponse, Response}
};
use opendal::Operator;

use crate::models::FileObjectMetadata;


pub async fn check_health() -> &'static str {
    "Everything looks good."
}

pub async fn upload_file(name: &str, ftype: &str) -> Result<(), Response> {
    println!("Name: {:?}\nType: {:?}", name, ftype);
    Ok(())
}

pub async fn get_all(op: Operator) -> Result<Vec<FileObjectMetadata>, &'static str> {
  let files = op.list("/").await.map_err(|_| "unable to read files")?;
  let metas = files
    .into_iter()
    .map(|file| FileObjectMetadata::try_from(file).map_err(|_| "unable to get metadata from files"))
    .collect::<Result<Vec<FileObjectMetadata>, &'static str>>()?;

  Ok(metas)
}