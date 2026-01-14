use std::time::SystemTime;

use opendal::{self, Metadata, raw::Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileObjectMetadata {
    pub key: String,
    pub size: u64,
    pub content_type: String,
    pub last_modified: Option<SystemTime>,
    pub etag: String,
}

impl TryFrom<(String, Metadata)> for FileObjectMetadata {
    type Error = String;
    fn try_from(value: (String, Metadata)) -> Result<Self, Self::Error> {
        Ok(Self {
            key: value.0.clone(),
            size: value.1.content_length(),
            content_type: value.1.content_type().unwrap_or("none").to_string(),
            last_modified: value
                .1
                .last_modified()
                .map(|t| SystemTime::from(Timestamp::from(t))),
            etag: value.1.etag().ok_or(format!("etag not found for key {}", value.0)).unwrap_or("none").to_string(),
        })
    }
}

impl TryFrom<opendal::Entry> for FileObjectMetadata {
    type Error = String;
    fn try_from(value: opendal::Entry) -> Result<Self, Self::Error> {
        let metadata = FileObjectMetadata::try_from((
            value.name().to_string(), value.metadata().to_owned()
        ))?;
        Ok(metadata)
    }
}


// pub struct FileCustomMetadata {}

// pub struct FileRawData {}

// pub struct FileModel {
//     pub name: String,
//     pub object_metadata: FileObjectMetadata,
//     pub metadata: FileMetadata,
//     pub raw_data: FileRawData,
// }