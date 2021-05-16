use crate::kinesis::entry::EncodedRecord;
use crate::limiter;
use crate::limiter::{Limiter, TokenBucket};

/// Enforces byte and record per second limits per-shard, this helps spread out load
/// in the event of large batch submissions
pub(crate) struct RecordLimiter {
    bytes: TokenBucket,
    records: TokenBucket,
}

impl RecordLimiter {
    pub fn new(records_per_second: u64, bytes_per_second: u64) -> RecordLimiter {
        RecordLimiter {
            bytes: TokenBucket::per_second(bytes_per_second),
            records: TokenBucket::per_second(records_per_second),
        }
    }
}

impl Limiter for RecordLimiter {
    type Item = EncodedRecord;

    fn active(&mut self) -> bool {
        self.records.active() || self.bytes.active()
    }

    fn try_take(&mut self, item: &Self::Item) -> Result<(), limiter::Error> {
        self.records.try_take(&1)?;
        self.bytes.try_take(&(item.bytes.len() as u64))
    }
}
