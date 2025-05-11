use std::{fmt, future::Future, sync::Arc};

use aws_config::SdkConfig;

#[derive(Clone, Debug)]
pub struct AppState<R: Repository> {
  repo: Arc<R>
}

impl<R: Repository> AppState<R> {

}

pub trait Repository: Clone + Send + Sync + 'static {
  fn get(&self, id: &str) -> impl Future<Output = Result<String, RepoError>> + Send;

  fn put(
      &self,
      id: &str,
      data: &str,
  ) -> impl Future<Output = Result<(), RepoError>> + Send;
}

#[derive(Debug)]
pub struct RepoError {
    pub message: String
}

impl fmt::Display for RepoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RepoError {
    fn description(&self) -> &str {
        &self.message
    }
}

pub struct S3Repo {
  client: aws_sdk_s3::Client
}

impl S3Repo {
  pub async fn new() -> Self {
    let config: SdkConfig = aws_config::load_from_env().await;
    let client: aws_sdk_s3::Client = aws_sdk_s3::Client::new(&config);
  
    Self{client}
  }
}

impl Repository for S3Repo {
  fn get(&self, id: &str) -> impl Future<Output = Result<String, RepoError>> + Send {
      
  }

  fn put(
        &self,
        id: &str,
        data: &str,
    ) -> impl Future<Output = Result<(), RepoError>> + Send {
      
  }
}