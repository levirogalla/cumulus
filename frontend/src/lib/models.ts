export type ServerFileObjectMetadata = {
  key: string,
  size: number,
  content_type: string,
  last_modified: Date,
  etag: string,
}

export type RawServerFileObjectMetadata = {
  key: string,
  size: number,
  content_type: string,
  last_modified: { secs_since_epoch: number; nanos_since_epoch: number },
  etag: string,
}