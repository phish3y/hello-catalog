use std::{fmt, future::Future, sync::Arc};

use aws_config::SdkConfig;
use aws_sdk_s3::{
    operation::get_object::{builders::GetObjectFluentBuilder, GetObjectOutput},
    primitives::{AggregatedBytes, ByteStream},
};
use rdkafka::{producer::FutureProducer, ClientConfig};

#[derive(Clone, Debug)]
pub struct AppState<R: Repository, N: Notifier> {
    pub repo: Arc<R>,
    pub notifier: Arc<N>,
}

impl<R: Repository, N: Notifier> AppState<R, N> {}

pub trait Repository: Clone + Send + Sync + 'static {
    fn get(&self, id: &str) -> impl Future<Output = Result<Vec<u8>, RepoError>> + Send;

    fn put(&self, id: &str, body: &[u8]) -> impl Future<Output = Result<(), RepoError>> + Send;
}

pub trait Notifier: Clone + Send + Sync + 'static {
    fn send(&self) -> impl Future<Output = Result<(), NotifierError>> + Send;
}

#[derive(Debug)]
pub struct RepoError {
    pub message: String,
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

#[derive(Debug)]
pub struct NotifierError {
    pub message: String,
}

impl fmt::Display for NotifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for NotifierError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Clone)]
pub struct S3Repo {
    client: aws_sdk_s3::Client,
    bucket: String,
}

impl S3Repo {
    pub async fn new(bucket: String) -> Self {
        let config: SdkConfig = aws_config::load_from_env().await;
        let client: aws_sdk_s3::Client = aws_sdk_s3::Client::new(&config);

        Self { client, bucket }
    }
}

impl Repository for S3Repo {
    fn get(&self, id: &str) -> impl Future<Output = Result<Vec<u8>, RepoError>> + Send {
        async move {
            let key: String = format!("{}/{}.zip", id, id);

            let req: GetObjectFluentBuilder =
                self.client.get_object().bucket(&self.bucket).key(&key);

            let resp: GetObjectOutput = req.send().await.map_err(|err| {
                if let Some(svc_err) = err.as_service_error() {
                    if svc_err.is_no_such_key() {
                        return RepoError {
                            message: format!("object not found: {}", key),
                        };
                    }
                }

                RepoError {
                    message: format!("failed to get object: {}, {}", key, err),
                }
            })?;

            let bytes: AggregatedBytes = resp.body.collect().await.map_err(|err| RepoError {
                message: format!("failed to collect object body: {}, {}", key, err),
            })?;

            Ok(bytes.into_bytes().to_vec())
        }
    }

    fn put(&self, id: &str, body: &[u8]) -> impl Future<Output = Result<(), RepoError>> + Send {
        async move {
            let key: String = format!("{}/{}.zip", id, id);
            self.client
                .put_object()
                .bucket(&self.bucket)
                .key(&key)
                .body(ByteStream::from(body.to_vec()))
                .send()
                .await
                .map_err(|err| RepoError {
                    message: format!("failed to put object: {}, {}", key, err),
                })?;

            Ok(())
        }
    }
}

#[derive(Clone)]
pub struct KafkaNotifier {
    producer: FutureProducer,
}

impl KafkaNotifier {
    pub async fn new(kafka_brokers: &str) -> Result<Self, NotifierError> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", kafka_brokers)
            // .set("message.timeout.ms", "5000")
            .create()
            .map_err(|err| NotifierError {
                message: format!("failed to crfeate kafka producer: {}", err),
            })?;

        Ok(Self { producer })
    }
}

impl Notifier for KafkaNotifier {
    fn send(&self) -> impl Future<Output = Result<(), NotifierError>> + Send {
        async move { todo!() }
    }
}
