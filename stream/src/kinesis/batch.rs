use crate::batch::Reducer;
use crate::kinesis::entry::EncodedRecord;
use observability_deps::tracing::info;

/// The RecordBatcher groups together multiple EncodedRecord so that they
/// can be submitted as a single PutRecords Kinesis API request
pub(crate) struct RecordBatcher {
    buffer: Vec<EncodedRecord>,
    cur_bytes: usize,
    max_bytes: usize,
    max_records: usize,
}

impl RecordBatcher {
    pub fn new(max_bytes: usize, max_records: usize) -> RecordBatcher {
        RecordBatcher {
            buffer: vec![],
            cur_bytes: 0,
            max_records,
            max_bytes,
        }
    }
}

impl Reducer for RecordBatcher {
    type Item = EncodedRecord;

    type Output = Vec<EncodedRecord>;

    fn try_push(&mut self, item: EncodedRecord) -> Option<EncodedRecord> {
        let new_bytes = self.cur_bytes.saturating_add(item.bytes.len());

        if self.buffer.len() >= self.max_records || new_bytes > self.max_bytes {
            info!("batch full");
            return Some(item);
        }

        self.cur_bytes = new_bytes;
        self.buffer.push(item);
        None
    }

    fn take(&mut self) -> Option<Self::Output> {
        if self.buffer.is_empty() {
            return None;
        }
        info!(
            bytes = self.cur_bytes,
            count = self.buffer.len(),
            "flushing batch"
        );

        self.cur_bytes = 0;
        Some(std::mem::take(self.buffer.as_mut()))
    }

    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
