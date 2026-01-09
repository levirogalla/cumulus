use std::sync::Arc;
use std::env;
use anyhow::Result;
use opendal::services::S3;
use opendal::Operator;
use api::utils::ENV_VARS;

#[tokio::main]
async fn main() -> Result<()> {
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

    let op: Operator = Operator::new(builder)?.finish();

    // op.write("test.txt", "Hello OpenDAL!").await?;
    let bs = op.read("test.txt").await?;
    let content = String::from_utf8(bs.to_vec())?;
    println!("read content: {}", content);
  
    Ok(())
}
