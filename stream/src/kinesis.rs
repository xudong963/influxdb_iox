use crate::kinesis::producer::KinesisProducer;
use hashbrown::HashMap;
use influxdb_line_protocol::OwnedParsedLine;
use parking_lot::Mutex;
use snafu::{ResultExt, Snafu};
use std::sync::Arc;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

mod batch;
mod entry;
mod limiter;
mod producer;
mod sink;
mod topology;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Producer error: {}", source))]
    ProducerError { source: producer::Error },
}
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct KinesisClient {
    client: rusoto_kinesis::KinesisClient,
    shutdown: tokio_util::sync::CancellationToken,
    producers: Mutex<HashMap<String, (Arc<KinesisProducer>, JoinHandle<()>)>>,
}

impl std::fmt::Debug for KinesisClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KinesisClient").finish()
    }
}

impl KinesisClient {
    pub fn new() -> Self {
        Self {
            client: kinesis_client(
                "".to_string(),
                Some("http://localhost:4567".to_string()),
                true,
            ),
            shutdown: CancellationToken::new(),
            producers: Default::default(),
        }
    }

    fn get_or_insert_producer(&self, arn: String) -> Arc<KinesisProducer> {
        let mut producers = self.producers.lock();
        let (producer, _) = producers
            .raw_entry_mut()
            .from_key(&arn)
            .or_insert_with(|| {
                let (producer, task) = KinesisProducer::new(
                    self.client.clone(),
                    arn.clone(),
                    self.shutdown.child_token(),
                );
                (arn, (Arc::new(producer), tokio::spawn(task)))
            })
            .1;
        Arc::clone(producer)
    }

    pub async fn handle_write(&self, arn: String, lines: Vec<OwnedParsedLine>) -> Result<()> {
        self.get_or_insert_producer(arn)
            .handle_write(lines)
            .await
            .context(ProducerError)
    }

    pub async fn drain(&self) {
        self.shutdown.cancel();

        let handles = {
            self.producers
                .lock()
                .drain()
                .map(|(_, (_, handle))| handle)
                .collect::<Vec<_>>()
        };

        for handle in handles.into_iter() {
            handle.await.expect("kinesis producer panicked")
        }
    }
}

// TODO: Clean this up
fn parse_region(region: String, endpoint: Option<String>) -> rusoto_core::Region {
    if let Some(endpoint) = endpoint {
        return rusoto_core::Region::Custom {
            name: region,
            endpoint,
        };
    }
    region.parse().expect("invalid region")
}

fn kinesis_client(
    region: String,
    endpoint: Option<String>,
    local: bool,
) -> rusoto_kinesis::KinesisClient {
    let region = parse_region(region, endpoint);
    let dispatcher =
        rusoto_core::request::HttpClient::new().expect("failed to create request dispatcher");

    if local {
        return rusoto_kinesis::KinesisClient::new_with(
            dispatcher,
            rusoto_core::credential::StaticProvider::new_minimal(
                "local".to_string(),
                "development".to_string(),
            ),
            region,
        );
    }

    rusoto_kinesis::KinesisClient::new_with(
        dispatcher,
        rusoto_core::credential::DefaultCredentialsProvider::new().unwrap(),
        region,
    )
}
