use std::io::Read;
use std::sync::Arc;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use flate2::read::GzDecoder;
use mutable_batch::MutableBatch;

use mutable_batch_lp::lines_to_batches;

fn generate_mutable_batch() -> (usize, Arc<MutableBatch>) {
    let raw = include_bytes!("../../tests/fixtures/lineproto/read_filter.lp.gz");
    let mut gz = GzDecoder::new(&raw[..]);

    let mut buffer = String::new();
    gz.read_to_string(&mut buffer).unwrap();

    let batches = lines_to_batches(&buffer, 0).unwrap();
    assert_eq!(batches.len(), 1);

    let batch = Arc::new(batches.into_iter().next().unwrap().1);
    (buffer.len(), batch)
}

pub fn extend(c: &mut Criterion) {
    let mut group = c.benchmark_group("extend");
    let (lp_bytes, src) = generate_mutable_batch();
    for count in &[1, 2, 3, 4, 5] {
        group.throughput(Throughput::Bytes(lp_bytes as u64 * *count as u64));
        group.bench_function(BenchmarkId::from_parameter(count), |b| {
            b.iter(|| {
                let mut dst = MutableBatch::new();
                for _ in 0..*count {
                    dst.extend_from(&src).unwrap();
                }
            });
        });
    }
    group.finish();
}

pub fn extend_ranges(c: &mut Criterion) {
    let mut group = c.benchmark_group("extend_ranges");
    let (lp_bytes, src) = generate_mutable_batch();

    let ranges: Vec<_> = (1..(src.rows())).step_by(2).map(|x| (x - 1)..x).collect();

    for count in &[1, 2, 3, 4, 5] {
        group.throughput(Throughput::Bytes(lp_bytes as u64 * *count as u64));
        group.bench_function(BenchmarkId::from_parameter(count), |b| {
            b.iter(|| {
                let mut dst = MutableBatch::new();
                for _ in 0..*count {
                    dst.extend_from_ranges(&src, &ranges).unwrap();
                }
            });
        });
    }
    group.finish();
}

criterion_group!(benches, extend, extend_ranges);
criterion_main!(benches);
