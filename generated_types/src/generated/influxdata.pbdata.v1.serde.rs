impl serde::Serialize for Column {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.column_name.is_empty() {
            len += 1;
        }
        if self.semantic_type != 0 {
            len += 1;
        }
        if self.values.is_some() {
            len += 1;
        }
        if !self.null_mask.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.pbdata.v1.Column", len)?;
        if !self.column_name.is_empty() {
            struct_ser.serialize_field("columnName", &self.column_name)?;
        }
        if self.semantic_type != 0 {
            let v = column::SemanticType::from_i32(self.semantic_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.semantic_type)))?;
            struct_ser.serialize_field("semanticType", &v)?;
        }
        if let Some(v) = self.values.as_ref() {
            struct_ser.serialize_field("values", v)?;
        }
        if !self.null_mask.is_empty() {
            struct_ser.serialize_field("nullMask", pbjson::private::base64::encode(&self.null_mask).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Column {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "columnName",
            "semanticType",
            "values",
            "nullMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ColumnName,
            SemanticType,
            Values,
            NullMask,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "columnName" => Ok(GeneratedField::ColumnName),
                            "semanticType" => Ok(GeneratedField::SemanticType),
                            "values" => Ok(GeneratedField::Values),
                            "nullMask" => Ok(GeneratedField::NullMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Column;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.pbdata.v1.Column")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Column, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut column_name = None;
                let mut semantic_type = None;
                let mut values = None;
                let mut null_mask = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ColumnName => {
                            if column_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnName"));
                            }
                            column_name = Some(map.next_value()?);
                        }
                        GeneratedField::SemanticType => {
                            if semantic_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("semanticType"));
                            }
                            semantic_type = Some(map.next_value::<column::SemanticType>()? as i32);
                        }
                        GeneratedField::Values => {
                            if values.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values = Some(map.next_value()?);
                        }
                        GeneratedField::NullMask => {
                            if null_mask.is_some() {
                                return Err(serde::de::Error::duplicate_field("nullMask"));
                            }
                            null_mask = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(Column {
                    column_name: column_name.unwrap_or_default(),
                    semantic_type: semantic_type.unwrap_or_default(),
                    values,
                    null_mask: null_mask.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.pbdata.v1.Column", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for column::SemanticType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SEMANTIC_TYPE_UNSPECIFIED",
            Self::Iox => "SEMANTIC_TYPE_IOX",
            Self::Tag => "SEMANTIC_TYPE_TAG",
            Self::Field => "SEMANTIC_TYPE_FIELD",
            Self::Time => "SEMANTIC_TYPE_TIME",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for column::SemanticType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SEMANTIC_TYPE_UNSPECIFIED",
            "SEMANTIC_TYPE_IOX",
            "SEMANTIC_TYPE_TAG",
            "SEMANTIC_TYPE_FIELD",
            "SEMANTIC_TYPE_TIME",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = column::SemanticType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(column::SemanticType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(column::SemanticType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SEMANTIC_TYPE_UNSPECIFIED" => Ok(column::SemanticType::Unspecified),
                    "SEMANTIC_TYPE_IOX" => Ok(column::SemanticType::Iox),
                    "SEMANTIC_TYPE_TAG" => Ok(column::SemanticType::Tag),
                    "SEMANTIC_TYPE_FIELD" => Ok(column::SemanticType::Field),
                    "SEMANTIC_TYPE_TIME" => Ok(column::SemanticType::Time),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for column::Values {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.i64_values.is_empty() {
            len += 1;
        }
        if !self.f64_values.is_empty() {
            len += 1;
        }
        if !self.u64_values.is_empty() {
            len += 1;
        }
        if !self.string_values.is_empty() {
            len += 1;
        }
        if !self.bool_values.is_empty() {
            len += 1;
        }
        if !self.bytes_values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.pbdata.v1.Column.Values", len)?;
        if !self.i64_values.is_empty() {
            struct_ser.serialize_field("i64Values", &self.i64_values.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.f64_values.is_empty() {
            struct_ser.serialize_field("f64Values", &self.f64_values)?;
        }
        if !self.u64_values.is_empty() {
            struct_ser.serialize_field("u64Values", &self.u64_values.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.string_values.is_empty() {
            struct_ser.serialize_field("stringValues", &self.string_values)?;
        }
        if !self.bool_values.is_empty() {
            struct_ser.serialize_field("boolValues", &self.bool_values)?;
        }
        if !self.bytes_values.is_empty() {
            struct_ser.serialize_field("bytesValues", &self.bytes_values.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for column::Values {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "i64Values",
            "f64Values",
            "u64Values",
            "stringValues",
            "boolValues",
            "bytesValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            I64Values,
            F64Values,
            U64Values,
            StringValues,
            BoolValues,
            BytesValues,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "i64Values" => Ok(GeneratedField::I64Values),
                            "f64Values" => Ok(GeneratedField::F64Values),
                            "u64Values" => Ok(GeneratedField::U64Values),
                            "stringValues" => Ok(GeneratedField::StringValues),
                            "boolValues" => Ok(GeneratedField::BoolValues),
                            "bytesValues" => Ok(GeneratedField::BytesValues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = column::Values;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.pbdata.v1.Column.Values")
            }

            fn visit_map<V>(self, mut map: V) -> Result<column::Values, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut i64_values = None;
                let mut f64_values = None;
                let mut u64_values = None;
                let mut string_values = None;
                let mut bool_values = None;
                let mut bytes_values = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::I64Values => {
                            if i64_values.is_some() {
                                return Err(serde::de::Error::duplicate_field("i64Values"));
                            }
                            i64_values = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                        GeneratedField::F64Values => {
                            if f64_values.is_some() {
                                return Err(serde::de::Error::duplicate_field("f64Values"));
                            }
                            f64_values = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                        GeneratedField::U64Values => {
                            if u64_values.is_some() {
                                return Err(serde::de::Error::duplicate_field("u64Values"));
                            }
                            u64_values = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                        GeneratedField::StringValues => {
                            if string_values.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValues"));
                            }
                            string_values = Some(map.next_value()?);
                        }
                        GeneratedField::BoolValues => {
                            if bool_values.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolValues"));
                            }
                            bool_values = Some(map.next_value()?);
                        }
                        GeneratedField::BytesValues => {
                            if bytes_values.is_some() {
                                return Err(serde::de::Error::duplicate_field("bytesValues"));
                            }
                            bytes_values = Some(
                                map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                    }
                }
                Ok(column::Values {
                    i64_values: i64_values.unwrap_or_default(),
                    f64_values: f64_values.unwrap_or_default(),
                    u64_values: u64_values.unwrap_or_default(),
                    string_values: string_values.unwrap_or_default(),
                    bool_values: bool_values.unwrap_or_default(),
                    bytes_values: bytes_values.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.pbdata.v1.Column.Values", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DatabaseBatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.database_name.is_empty() {
            len += 1;
        }
        if !self.table_batches.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.pbdata.v1.DatabaseBatch", len)?;
        if !self.database_name.is_empty() {
            struct_ser.serialize_field("databaseName", &self.database_name)?;
        }
        if !self.table_batches.is_empty() {
            struct_ser.serialize_field("tableBatches", &self.table_batches)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DatabaseBatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "databaseName",
            "tableBatches",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DatabaseName,
            TableBatches,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "databaseName" => Ok(GeneratedField::DatabaseName),
                            "tableBatches" => Ok(GeneratedField::TableBatches),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DatabaseBatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.pbdata.v1.DatabaseBatch")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DatabaseBatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut database_name = None;
                let mut table_batches = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DatabaseName => {
                            if database_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("databaseName"));
                            }
                            database_name = Some(map.next_value()?);
                        }
                        GeneratedField::TableBatches => {
                            if table_batches.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableBatches"));
                            }
                            table_batches = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DatabaseBatch {
                    database_name: database_name.unwrap_or_default(),
                    table_batches: table_batches.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.pbdata.v1.DatabaseBatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TableBatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.table_name.is_empty() {
            len += 1;
        }
        if !self.columns.is_empty() {
            len += 1;
        }
        if self.row_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.pbdata.v1.TableBatch", len)?;
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        if self.row_count != 0 {
            struct_ser.serialize_field("rowCount", &self.row_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TableBatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tableName",
            "columns",
            "rowCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableName,
            Columns,
            RowCount,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tableName" => Ok(GeneratedField::TableName),
                            "columns" => Ok(GeneratedField::Columns),
                            "rowCount" => Ok(GeneratedField::RowCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TableBatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.pbdata.v1.TableBatch")
            }

            fn visit_map<V>(self, mut map: V) -> Result<TableBatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_name = None;
                let mut columns = None;
                let mut row_count = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TableName => {
                            if table_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name = Some(map.next_value()?);
                        }
                        GeneratedField::Columns => {
                            if columns.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns = Some(map.next_value()?);
                        }
                        GeneratedField::RowCount => {
                            if row_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowCount"));
                            }
                            row_count = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(TableBatch {
                    table_name: table_name.unwrap_or_default(),
                    columns: columns.unwrap_or_default(),
                    row_count: row_count.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.pbdata.v1.TableBatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.database_batch.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.pbdata.v1.WriteRequest", len)?;
        if let Some(v) = self.database_batch.as_ref() {
            struct_ser.serialize_field("databaseBatch", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "databaseBatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DatabaseBatch,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "databaseBatch" => Ok(GeneratedField::DatabaseBatch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.pbdata.v1.WriteRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<WriteRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut database_batch = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DatabaseBatch => {
                            if database_batch.is_some() {
                                return Err(serde::de::Error::duplicate_field("databaseBatch"));
                            }
                            database_batch = Some(map.next_value()?);
                        }
                    }
                }
                Ok(WriteRequest {
                    database_batch,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.pbdata.v1.WriteRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.pbdata.v1.WriteResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.pbdata.v1.WriteResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<WriteResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(WriteResponse {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.pbdata.v1.WriteResponse", FIELDS, GeneratedVisitor)
    }
}
