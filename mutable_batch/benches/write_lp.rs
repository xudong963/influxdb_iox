use std::io::Read;

use bytes::Bytes;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use flate2::read::GzDecoder;

use influxdb_line_protocol::parse_lines;
use mutable_batch::MutableBatch;

fn generate_lp_bytes() -> Bytes {
    let raw = include_bytes!("../../tests/fixtures/lineproto/read_filter.lp.gz");
    let mut gz = GzDecoder::new(&raw[..]);

    let mut buffer = Vec::new();
    gz.read_to_end(&mut buffer).unwrap();
    buffer.into()
}

pub fn write_lp(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_lp");
    let lp_bytes = generate_lp_bytes();
    for count in &[1, 2, 3, 4, 5] {
        group.bench_function(BenchmarkId::from_parameter(count), |b| {
            b.iter(|| {
                let mut mb = MutableBatch::new();

                for _ in 0..*count {
                    for line in parse_lines(std::str::from_utf8(&lp_bytes).unwrap()) {
                        mb.write_line(line.unwrap(), 0).unwrap()
                    }
                }
            });
        });
    }
    group.finish();
}

criterion_group!(benches, write_lp);
criterion_main!(benches);
