use std::io::Read;

use bytes::{Bytes, BytesMut};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use generated_types::influxdata::pbdata::v1::TableBatch;
use influxdb_line_protocol::parse_lines;
use mutable_batch::MutableBatch;
use prost::Message;

fn generate_pb_bytes() -> (usize, Bytes) {
    let lp = std::fs::read_to_string("/home/raphael/Downloads/real_data_mem.lp").unwrap();

    let mut batch = MutableBatch::new();
    for line in parse_lines(&lp) {
        batch.write_line(line.unwrap(), 0).unwrap();
    }

    let mut buf = BytesMut::new();
    let pb = batch.to_pb("test".to_string());
    pb.encode(&mut buf).unwrap();
    (lp.len(), buf.freeze())
}

pub fn write_pb(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_pb");
    let (lp_len, bytes) = generate_pb_bytes();
    let bytes: &[u8] = &bytes;

    for count in [1, 2, 3, 4, 5] {
        group.throughput(Throughput::Bytes(lp_len as u64 * count as u64));
        group.bench_function(BenchmarkId::from_parameter(count), |b| {
            b.iter(|| {
                let mut mb = MutableBatch::new();

                for _ in 0..count {
                    let decoded = TableBatch::decode(bytes).unwrap();
                    mb.write_pb(decoded).unwrap()
                }
            });
        });
    }
    group.finish();
}

criterion_group!(benches, write_pb);
criterion_main!(benches);
