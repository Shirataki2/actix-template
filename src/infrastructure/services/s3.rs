use std::time::Duration;

use aws_sdk_s3::operation::put_object::PutObjectOutput;

use crate::{error::Error, infrastructure::data::AppData};

#[async_trait]
trait UploadS3 {
    async fn upload_s3(&self, key: String, body: Vec<u8>) -> Result<PutObjectOutput, Error>;
}

#[async_trait]
impl UploadS3 for AppData {
    async fn upload_s3(&self, key: String, body: Vec<u8>) -> Result<PutObjectOutput, Error> {
        let bucket = &self.settings.s3_bucket;
        let s3_client = &self.s3_client;
        let body = aws_sdk_s3::primitives::ByteStream::from(body);
        s3_client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(body)
            .send()
            .await
            .map_err(|e| Error::S3(e.to_string()))
    }
}

#[cfg(test)]
mod test_upload_s3 {
    use super::*;
    use crate::infrastructure::config::Settings;

    #[tokio::test]
    async fn test_upload_s3() {
        let settings = Settings::new().unwrap();
        let app_data = AppData::create(&settings).await.unwrap();
        let key = "test.txt".to_string();
        let body = "test".as_bytes().to_vec();
        let result = app_data.upload_s3(key, body).await;
        println!("{:?}", result);
    }
}

#[async_trait]
trait IssuePresignedUrl {
    async fn issue_presigned_url(
        &self,
        key: String,
        expires_in: Option<Duration>,
    ) -> Result<String, Error>;
}

#[async_trait]
impl IssuePresignedUrl for AppData {
    async fn issue_presigned_url(
        &self,
        key: String,
        expires_in: Option<Duration>,
    ) -> Result<String, Error> {
        let bucket = &self.settings.s3_bucket;
        let s3_client = &self.s3_client;
        let config = aws_sdk_s3::presigning::PresigningConfig::builder()
            .expires_in(expires_in.unwrap_or(Duration::from_secs(60 * 60 * 24 * 7)))
            .build()
            .map_err(|e| Error::S3(e.to_string()))?;
        let req = s3_client
            .get_object()
            .bucket(bucket)
            .key(key)
            .presigned(config)
            .await
            .map_err(|e| Error::S3(e.to_string()))?;
        let url = req.uri().to_string();
        Ok(url)
    }
}

#[cfg(test)]
mod test_presigned_url {
    use super::*;
    use crate::infrastructure::config::Settings;

    #[tokio::test]
    async fn test_issue_presigned_url() {
        let settings = Settings::new().unwrap();
        let app_data = AppData::create(&settings).await.unwrap();
        let key = "test.txt".to_string();
        let result = app_data.issue_presigned_url(key, None).await;
        println!("{:?}", result);
    }
}
