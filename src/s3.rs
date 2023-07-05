use aws_sdk_s3::presigning::{config::PresigningConfig, request::PresignedRequest};
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{Client, Error};

use std::time::Duration;

pub async fn get_aws_client() -> Client {
    let config = aws_config::load_from_env().await;
    Client::new(&config)
}

pub async fn updload_object(bucket: &str, file: Vec<u8>, key: &str) -> Result<(), Error> {
    let client = get_aws_client().await;
    let body = ByteStream::from(file);

    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .send()
        .await?;

    Ok(())
}

pub async fn object_exist(bucket: &str, key: &str) -> bool {
    let client = get_aws_client().await;

    let resp = client
        .list_objects_v2()
        .bucket(bucket)
        .prefix(key)
        .send()
        .await
        .unwrap_or_else(|error| panic!("{:?}", error));
    resp.contents().is_some()
}

pub async fn get_signed_url(
    bucket: &str,
    key: &str,
) -> Result<PresignedRequest, Box<dyn std::error::Error>> {
    let client = get_aws_client().await;
    let expires_in = Duration::from_secs(144000);
    let resp = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .presigned(PresigningConfig::expires_in(expires_in)?)
        .await?;
    Ok(resp)
}
