use influxdb_line_protocol::OwnedParsedLine;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {}
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct KinesisClient {}

impl KinesisClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn handle_write(&self, _arn: String, _lines: Vec<OwnedParsedLine>) -> Result<()> {
        todo!()
    }

    pub async fn drain(&self) {

    }
}
