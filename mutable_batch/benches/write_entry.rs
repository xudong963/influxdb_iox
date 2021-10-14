use std::convert::TryInto;
use std::io::Read;

use bytes::Bytes;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use entry::{
    test_helpers::{hour_partitioner, lp_to_entries},
    Entry,
};
use mutable_batch::MutableBatch;

fn generate_entry_bytes() -> (usize, Bytes) {
    let lp = std::fs::read_to_string("/home/raphael/Downloads/real_data_mem.lp").unwrap();

    let lp_len = lp.len();

    let entries = lp_to_entries(&lp, &hour_partitioner());
    assert_eq!(entries.len(), 1);

    (lp_len, entries.into_iter().next().unwrap().into())
}

pub fn write_entry(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_entry");
    let (lp_len, entry_bytes) = generate_entry_bytes();
    for count in [1, 2, 3, 4, 5] {
        group.throughput(Throughput::Bytes(lp_len as u64 * count as u64));
        group.bench_function(BenchmarkId::from_parameter(count), |b| {
            b.iter(|| {
                let mut mb = MutableBatch::new();

                for _ in 0..count {
                    let entry: Entry = entry_bytes.clone().try_into().unwrap();

                    for write in entry.partition_writes().iter().flatten() {
                        for batch in write.table_batches() {
                            mb.write_table_batch(batch, None).unwrap();
                        }
                    }
                }
            });
        });
    }
    group.finish();
}

criterion_group!(benches, write_entry);
criterion_main!(benches);
