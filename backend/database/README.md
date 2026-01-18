# Database Tables

media {
  key: varchar
  notes: varchar
  albums: albums_media
  date_uploaded: date
}

albums {
  id: number
  name: varchar
  posts: albums_media
  date_created: date
}

albums_media {
  id: number
  media: media.key
  album: album.key
  date_added: date
}