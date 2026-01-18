use std::time::SystemTime;

use opendal::{self, Metadata, raw::Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileObjectMetadata {
    pub key: String,
    pub size: u64,
    // pub content_type: String,
    pub last_modified: Option<SystemTime>,
    pub etag: String,
}

impl TryFrom<(String, Metadata)> for FileObjectMetadata {
    type Error = String;
    fn try_from(value: (String, Metadata)) -> Result<Self, Self::Error> {
        Ok(Self {
            key: value.0.clone(),
            size: value.1.content_length(),
            // content_type: value.1.content_type().unwrap_or("none").to_string(),
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

pub mod database {
    use std::time::SystemTime;
    use diesel::prelude::*;

   #[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)] 
   #[diesel(table_name=crate::schema::albums)]
   pub struct Album {
    pub id: i32,
    pub name: String,
    pub date_created: SystemTime 
   }

   #[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)] 
   #[diesel(table_name=crate::schema::albums_media)]
   #[diesel(belongs_to(Album, foreign_key=album))]
   #[diesel(belongs_to(Media, foreign_key=media))]
   pub struct AlbumsMedia {
    pub id: i32,
    pub media: String,
    pub album: String,
    pub date_added: Option<SystemTime>,
   }


   #[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)] 
   #[diesel(table_name=crate::schema::media)]
   #[diesel(primary_key(key))]
   pub struct Media {
    pub key: String,
    pub date_uploaded: Option<SystemTime>
   }
}
