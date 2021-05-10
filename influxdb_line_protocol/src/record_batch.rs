use std::sync::Arc;

use arrow::datatypes::DataType;
use arrow::record_batch::RecordBatch;
use arrow::{
    array::{Array, Float64Array, Int64Array, PrimitiveArray, StringArray, UInt64Array},
    datatypes::Int64Type,
};

#[derive(Debug)]
pub struct RecordBatchConverter {
    rb: RecordBatch,
    measurement: &'static str,
}

impl RecordBatchConverter {
    pub fn new(table_name: &'static str, rb: RecordBatch) -> Self {
        Self {
            rb,
            measurement: table_name,
        }
    }

    // TODO(edd): use meta data that describes IOx semantic column types.
    pub fn convert(&self) -> &str {
        let total_rows = self.rb.num_rows();
        let mut tag_names: Vec<(String, Box<dyn ValueWriter>)> = vec![];
        let mut field_names: Vec<(String, Box<dyn ValueWriter>)> = vec![];

        // use semantic column type instead
        for (i, field) in self.rb.schema().fields().iter().enumerate() {
            match field.data_type() {
                DataType::Int64 => {
                    let arr = self
                        .rb
                        .column(i)
                        .as_any()
                        .downcast_ref::<Int64Array>()
                        .unwrap();

                    field_names.push((
                        field.name().to_owned(),
                        Box::new(arr) as Box<dyn ValueWriter>,
                    ));
                }
                DataType::UInt64 => {
                    let arr = self
                        .rb
                        .column(i)
                        .as_any()
                        .downcast_ref::<UInt64Array>()
                        .unwrap();
                    field_names.push((field.name().to_owned(), Box::new(arr as _)));
                }
                DataType::Float64 => {
                    let arr = self
                        .rb
                        .column(i)
                        .as_any()
                        .downcast_ref::<Float64Array>()
                        .unwrap();
                    field_names.push((field.name().to_owned(), Box::new(arr as _)));
                }
                DataType::Utf8 => {
                    let arr = self
                        .rb
                        .column(i)
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap();

                    tag_names.push((field.name().to_owned(), Box::new(arr as _)));
                }
                DataType::Dictionary(_, _) => {
                    let arr = self
                        .rb
                        .column(i)
                        .as_any()
                        .downcast_ref::<StringArray>()
                        .unwrap();

                    // dunno why `as _` worked.
                    tag_names.push((field.name().to_owned(), Box::new(arr as _)));
                }
                _ => panic!("unsupported type"),
            }
        }

        let mut lines = String::with_capacity(100000);
        let mut row = 0;

        while row < total_rows {
            lines.push_str(self.measurement);
            lines.push_str(", ");

            for (i, (tk, value_writer)) in tag_names.iter().enumerate() {
                if value_writer.is_null(row) {
                    continue;
                }

                lines.push_str(tk.as_str());
                lines.push_str("=");
                value_writer.push_value(row, &mut lines);

                if i < tag_names.len() - 1 {
                    lines.push_str(",");
                }
            }

            lines.push_str(" ");

            for (i, (fk, value_writer)) in field_names.iter().enumerate() {
                if value_writer.is_null(row) {
                    continue;
                }

                lines.push_str(fk.as_str());
                lines.push_str("=");
                value_writer.push_value(row, &mut lines);

                if i < tag_names.len() - 1 {
                    lines.push_str(",");
                }
            }

            lines.push('\n');
            row += 1;
        }

        ""
    }
}

trait ValueWriter: Array {
    fn push_value(&self, i: usize, s: &mut String);
}

impl ValueWriter for StringArray {
    fn push_value(&self, i: usize, s: &mut String) {
        s.push_str(self.value(i));
    }
}

impl ValueWriter for Int64Array {
    fn push_value(&self, i: usize, s: &mut String) {
        s.push_str(format!("{:?}", self.value(i)).as_str());
    }
}
