use dotenv::dotenv;
use opendal::{Operator, services::S3};
use std::env;

// env vars
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

pub fn get_file_op() -> Option<Operator> {
    let env_vars = ENV_VARS::get();
    // Create s3 backend builder.
    let builder = S3::default()
        // Set the root for s3, all operations will happen under this root.
        //
        // NOTE: the root must be absolute path.
        .root("/files")
        // Set the bucket name. This is required.
        .bucket(env_vars.s3_bucket_name.as_str())
        // Set the region. This is required for some services, if you don't care about it, for example Minio service, just set it to "auto", it will be ignored.
        .region(env_vars.s3_region.as_str())
        // Set the endpoint.
        //
        // For examples:
        // - "https://s3.amazonaws.com"
        // - "http://127.0.0.1:9000"
        // - "https://oss-ap-northeast-1.aliyuncs.com"
        // - "https://cos.ap-seoul.myqcloud.com"
        //
        // Default to "https://s3.amazonaws.com"
        .endpoint(env_vars.s3_bucket_url.as_str())
        // Set the access_key_id and secret_access_key.
        //
        // OpenDAL will try load credential from the env.
        // If credential not set and no valid credential in env, OpenDAL will
        // send request without signing like anonymous user.
        .access_key_id(env_vars.s3_key_id.as_str())
        .secret_access_key(env_vars.s3_key_secret.as_str());

    Operator::new(builder).map(|op| op.finish()).ok()
}

pub fn get_media_op() -> Option<Operator> {
    let env_vars = ENV_VARS::get();
    // Create s3 backend builder.
    let builder = S3::default()
        // Set the root for s3, all operations will happen under this root.
        //
        // NOTE: the root must be absolute path.
        .root("/media")
        // Set the bucket name. This is required.
        .bucket(env_vars.s3_bucket_name.as_str())
        // Set the region. This is required for some services, if you don't care about it, for example Minio service, just set it to "auto", it will be ignored.
        .region(env_vars.s3_region.as_str())
        // Set the endpoint.
        //
        // For examples:
        // - "https://s3.amazonaws.com"
        // - "http://127.0.0.1:9000"
        // - "https://oss-ap-northeast-1.aliyuncs.com"
        // - "https://cos.ap-seoul.myqcloud.com"
        //
        // Default to "https://s3.amazonaws.com"
        .endpoint(env_vars.s3_bucket_url.as_str())
        // Set the access_key_id and secret_access_key.
        //
        // OpenDAL will try load credential from the env.
        // If credential not set and no valid credential in env, OpenDAL will
        // send request without signing like anonymous user.
        .access_key_id(env_vars.s3_key_id.as_str())
        .secret_access_key(env_vars.s3_key_secret.as_str());

    Operator::new(builder).map(|op| op.finish()).ok()
}
