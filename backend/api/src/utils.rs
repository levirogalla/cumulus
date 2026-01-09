use dotenv::dotenv;
use std::env;
pub struct ENV_VARS {
    pub s3_key_id: String,
    pub s3_key_name: String,
    pub s3_key_secret: String,
    pub s3_region: String,
    pub s3_bucket_name: String,
    pub s3_bucket_url: String,
}

impl ENV_VARS {
    pub fn get() -> Self {
        dotenv().ok();
        Self {
            s3_key_id: env::var("S3_KEY_ID").expect("S3_KEY_ID not set"),
            s3_key_name: env::var("S3_KEY_NAME").expect("S3_KEY_NAME not set"),
            s3_key_secret: env::var("S3_KEY_SECRET").expect("S3_KEY_SECRET not set"),
            s3_region: env::var("S3_REGION").expect("S3_REGION not set"),
            s3_bucket_name: env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME not set"),
            s3_bucket_url: env::var("S3_BUCKET_URL").expect("S3_BUCKET_URL not set"),
        }
    }
}
