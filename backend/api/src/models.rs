use std::time::SystemTime;

use opendal::{self, raw::Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileObjectMetadata {
    pub key: String,
    pub size: u64,
    pub content_type: String,
    pub last_modified: Option<SystemTime>,
    pub etag: String,
}

impl TryFrom<opendal::Entry> for FileObjectMetadata {
    type Error = ();
    fn try_from(value: opendal::Entry) -> Result<Self, Self::Error> {
        Ok(Self {
            key: value.path().to_string(),
            size: value.metadata().content_length(),
            content_type: value.metadata().content_type().unwrap_or("none").to_string(),
            last_modified: value
                .metadata()
                .last_modified()
                .map(|t| SystemTime::from(Timestamp::from(t))),
            etag: value.metadata().etag().unwrap_or_default().to_string(),
        })
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