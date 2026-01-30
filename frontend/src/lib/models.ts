export type ServerFileObjectMetadata = {
	key: string;
	size: number;
	// content_type: string,
	last_modified: Date | null;
	etag: string;
};

export type RawServerFileObjectMetadata = {
	key: string;
	size: number;
	// content_type: string,
	last_modified: { secs_since_epoch: number; nanos_since_epoch: number } | null;
	etag: string;
};

export type BucketName = 'files' | 'media';
