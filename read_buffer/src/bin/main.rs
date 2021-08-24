use datafusion::parquet::arrow::{ArrowReader, ParquetFileArrowReader};
use datafusion::parquet::file::serialized_reader::SerializedFileReader;
use std::fs::File;
use std::sync::Arc;

fn main() {
    let file = File::open("/Users/edd/Downloads/chunk.parquet").unwrap();
    let file_reader = SerializedFileReader::new(file).unwrap();
    let mut arrow_reader = ParquetFileArrowReader::new(Arc::new(file_reader));

    // 1024*25 is current row group size
    let record_batch_reader = arrow_reader.get_record_reader(1024 * 25).unwrap();
    let mut itr = record_batch_reader;
    let cm = read_buffer::ChunkMetrics::new_unregistered();
    let mut chunk = read_buffer::RBChunk::new("my_table", itr.next().unwrap().unwrap(), cm);

    for rb in itr {
        chunk.upsert_table(rb.unwrap());
    }

    println!("{:?}", chunk.table_summary());
}
