use crate::batch::{Partitioned, Reducer};
use crate::kinesis::producer::LineRecord;
use crate::kinesis::topology::{ShardId, TopologyGeneration};
use bytes::{Bytes, BytesMut};

mod influxdata {
    pub mod iox {
        pub mod kinesis {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/influxdata.iox.kinesis.v1.rs"));
            }
        }
    }
}

use data_types::database_rules::PartitionTemplate;
use entry::lines_to_entry;
use influxdata::iox::kinesis::v1::*;

/// The encoded data written to kinesis
///
/// This is an aggregation of multiple LineRecords destined for the same shard
pub struct EncodedRecord {
    /// This is an arbitrary hash key of one of the LineRecord's aggregated into this
    pub hash_key: u128,

    /// The shard where this record is predicted to end up, the producer will aggregate
    /// multiple writes to different hash keys that map to the same shard in the same
    /// encoded payload.
    ///
    /// This expected shard id is also encoded in the data payload. A consumer MUST
    /// reject data payloads that contain a shard ID other than where the payload
    /// was sourced from.
    ///
    /// This implies the producer had an outdated shard map, which can occur during
    /// resharding operations. A producer will detect this, update its shard map and
    /// re-aggregate the source writes according to the new shard map
    ///
    /// This ensures that a consistent order is established for a given hash key
    pub predicted_shard_id: (ShardId, TopologyGeneration),

    /// The encoded bytes of the payload
    pub bytes: Bytes,

    /// These must be retained in case of an error
    pub lines: Vec<LineRecord>,
}

impl EncodedRecord {
    pub fn ack(self, res: Result<(), crate::kinesis::producer::Error>) {
        for line in self.lines {
            line.ack(res.clone())
        }
    }
}

impl Partitioned for EncodedRecord {
    type Key = ShardId;

    fn partition(&self) -> Self::Key {
        self.predicted_shard_id.0
    }
}

/// The EntryReducer compacts multiple LineRecord's with the same predicted shard
/// into a single EncodedRecord
pub struct EntryReducer {
    max_bytes: usize,
    cur_bytes: usize,
    buffer: Vec<LineRecord>,
}

impl EntryReducer {
    pub fn new(max_bytes: usize) -> Self {
        Self {
            max_bytes,
            cur_bytes: 0,
            buffer: vec![],
        }
    }
}

impl Reducer for EntryReducer {
    type Item = LineRecord;
    type Output = EncodedRecord;

    fn try_push(&mut self, item: Self::Item) -> Option<Self::Item> {
        if item.line.size_bytes() + self.cur_bytes > self.max_bytes {
            return Some(item);
        }
        self.buffer.push(item);
        None
    }

    fn take(&mut self) -> Option<Self::Output> {
        if self.buffer.is_empty() {
            return None;
        }

        let lines = {
            let mut lines = Vec::new();
            std::mem::swap(&mut self.buffer, &mut lines);
            lines
        };

        // TODO: Error handling
        // Need a mechanism to reject individual lines without failing the entire batch
        let entry: Vec<u8> = lines_to_entry(
            lines.iter().map(|x| x.line.inner()),
            &PartitionTemplate::default(),
            &chrono::Utc::now(),
        ).unwrap().into();

        let expected_shard = lines[0].partition().to_string();
        let payload = EntryPayload {
            entry: entry.into(),
            expected_shard,
        };


        let mut encoded = BytesMut::new();

        prost::Message::encode(&payload, &mut encoded).unwrap();
        Some(EncodedRecord {
            hash_key: 0,
            predicted_shard_id: lines[0].predicted_shard_id.unwrap(),
            bytes: encoded.freeze(),
            lines,
        })
    }

    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
