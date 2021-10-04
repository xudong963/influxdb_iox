impl serde::Serialize for AddParquet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.path.is_some() {
            len += 1;
        }
        if self.file_size_bytes != 0 {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.AddParquet", len)?;
        if let Some(v) = self.path.as_ref() {
            struct_ser.serialize_field("path", v)?;
        }
        if self.file_size_bytes != 0 {
            struct_ser.serialize_field("fileSizeBytes", ToString::to_string(&self.file_size_bytes).as_str())?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", pbjson::private::base64::encode(&self.metadata).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddParquet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "fileSizeBytes",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            FileSizeBytes,
            Metadata,
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
                            "path" => Ok(GeneratedField::Path),
                            "fileSizeBytes" => Ok(GeneratedField::FileSizeBytes),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddParquet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.AddParquet")
            }

            fn visit_map<V>(self, mut map: V) -> Result<AddParquet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path = None;
                let mut file_size_bytes = None;
                let mut metadata = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path = Some(map.next_value()?);
                        }
                        GeneratedField::FileSizeBytes => {
                            if file_size_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileSizeBytes"));
                            }
                            file_size_bytes = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Metadata => {
                            if metadata.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(AddParquet {
                    path,
                    file_size_bytes: file_size_bytes.unwrap_or_default(),
                    metadata: metadata.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.AddParquet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChunkAddr {
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
        if !self.partition_key.is_empty() {
            len += 1;
        }
        if self.chunk_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.ChunkAddr", len)?;
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if !self.partition_key.is_empty() {
            struct_ser.serialize_field("partitionKey", &self.partition_key)?;
        }
        if self.chunk_id != 0 {
            struct_ser.serialize_field("chunkId", &self.chunk_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChunkAddr {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tableName",
            "partitionKey",
            "chunkId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableName,
            PartitionKey,
            ChunkId,
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
                            "partitionKey" => Ok(GeneratedField::PartitionKey),
                            "chunkId" => Ok(GeneratedField::ChunkId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChunkAddr;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.ChunkAddr")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ChunkAddr, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_name = None;
                let mut partition_key = None;
                let mut chunk_id = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TableName => {
                            if table_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name = Some(map.next_value()?);
                        }
                        GeneratedField::PartitionKey => {
                            if partition_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionKey"));
                            }
                            partition_key = Some(map.next_value()?);
                        }
                        GeneratedField::ChunkId => {
                            if chunk_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunkId"));
                            }
                            chunk_id = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(ChunkAddr {
                    table_name: table_name.unwrap_or_default(),
                    partition_key: partition_key.unwrap_or_default(),
                    chunk_id: chunk_id.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.ChunkAddr", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DatabaseCheckpoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sequencer_numbers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.DatabaseCheckpoint", len)?;
        if !self.sequencer_numbers.is_empty() {
            struct_ser.serialize_field("sequencerNumbers", &self.sequencer_numbers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DatabaseCheckpoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sequencerNumbers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SequencerNumbers,
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
                            "sequencerNumbers" => Ok(GeneratedField::SequencerNumbers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DatabaseCheckpoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.DatabaseCheckpoint")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DatabaseCheckpoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sequencer_numbers = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SequencerNumbers => {
                            if sequencer_numbers.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequencerNumbers"));
                            }
                            sequencer_numbers = Some(
                                map.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<u32>, _>>()?
                                    .into_iter().map(|(k,v)| (k.0, v)).collect()
                            );
                        }
                    }
                }
                Ok(DatabaseCheckpoint {
                    sequencer_numbers: sequencer_numbers.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.DatabaseCheckpoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeletePredicate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.predicate.is_some() {
            len += 1;
        }
        if !self.chunks.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.DeletePredicate", len)?;
        if let Some(v) = self.predicate.as_ref() {
            struct_ser.serialize_field("predicate", v)?;
        }
        if !self.chunks.is_empty() {
            struct_ser.serialize_field("chunks", &self.chunks)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeletePredicate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "predicate",
            "chunks",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Predicate,
            Chunks,
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
                            "predicate" => Ok(GeneratedField::Predicate),
                            "chunks" => Ok(GeneratedField::Chunks),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeletePredicate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.DeletePredicate")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DeletePredicate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut predicate = None;
                let mut chunks = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Predicate => {
                            if predicate.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate = Some(map.next_value()?);
                        }
                        GeneratedField::Chunks => {
                            if chunks.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunks"));
                            }
                            chunks = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DeletePredicate {
                    predicate,
                    chunks: chunks.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.DeletePredicate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Expr {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.column.is_empty() {
            len += 1;
        }
        if self.op != 0 {
            len += 1;
        }
        if self.scalar.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.Expr", len)?;
        if !self.column.is_empty() {
            struct_ser.serialize_field("column", &self.column)?;
        }
        if self.op != 0 {
            let v = Op::from_i32(self.op)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.op)))?;
            struct_ser.serialize_field("op", &v)?;
        }
        if let Some(v) = self.scalar.as_ref() {
            struct_ser.serialize_field("scalar", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Expr {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "column",
            "op",
            "scalar",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Column,
            Op,
            Scalar,
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
                            "column" => Ok(GeneratedField::Column),
                            "op" => Ok(GeneratedField::Op),
                            "scalar" => Ok(GeneratedField::Scalar),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Expr;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.Expr")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Expr, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut column = None;
                let mut op = None;
                let mut scalar = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Column => {
                            if column.is_some() {
                                return Err(serde::de::Error::duplicate_field("column"));
                            }
                            column = Some(map.next_value()?);
                        }
                        GeneratedField::Op => {
                            if op.is_some() {
                                return Err(serde::de::Error::duplicate_field("op"));
                            }
                            op = Some(map.next_value::<Op>()? as i32);
                        }
                        GeneratedField::Scalar => {
                            if scalar.is_some() {
                                return Err(serde::de::Error::duplicate_field("scalar"));
                            }
                            scalar = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Expr {
                    column: column.unwrap_or_default(),
                    op: op.unwrap_or_default(),
                    scalar,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.Expr", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IoxMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if self.creation_timestamp.is_some() {
            len += 1;
        }
        if !self.table_name.is_empty() {
            len += 1;
        }
        if !self.partition_key.is_empty() {
            len += 1;
        }
        if self.chunk_id != 0 {
            len += 1;
        }
        if self.partition_checkpoint.is_some() {
            len += 1;
        }
        if self.database_checkpoint.is_some() {
            len += 1;
        }
        if self.time_of_first_write.is_some() {
            len += 1;
        }
        if self.time_of_last_write.is_some() {
            len += 1;
        }
        if self.chunk_order != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.IoxMetadata", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if let Some(v) = self.creation_timestamp.as_ref() {
            struct_ser.serialize_field("creationTimestamp", v)?;
        }
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if !self.partition_key.is_empty() {
            struct_ser.serialize_field("partitionKey", &self.partition_key)?;
        }
        if self.chunk_id != 0 {
            struct_ser.serialize_field("chunkId", &self.chunk_id)?;
        }
        if let Some(v) = self.partition_checkpoint.as_ref() {
            struct_ser.serialize_field("partitionCheckpoint", v)?;
        }
        if let Some(v) = self.database_checkpoint.as_ref() {
            struct_ser.serialize_field("databaseCheckpoint", v)?;
        }
        if let Some(v) = self.time_of_first_write.as_ref() {
            struct_ser.serialize_field("timeOfFirstWrite", v)?;
        }
        if let Some(v) = self.time_of_last_write.as_ref() {
            struct_ser.serialize_field("timeOfLastWrite", v)?;
        }
        if self.chunk_order != 0 {
            struct_ser.serialize_field("chunkOrder", &self.chunk_order)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IoxMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "creationTimestamp",
            "tableName",
            "partitionKey",
            "chunkId",
            "partitionCheckpoint",
            "databaseCheckpoint",
            "timeOfFirstWrite",
            "timeOfLastWrite",
            "chunkOrder",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            CreationTimestamp,
            TableName,
            PartitionKey,
            ChunkId,
            PartitionCheckpoint,
            DatabaseCheckpoint,
            TimeOfFirstWrite,
            TimeOfLastWrite,
            ChunkOrder,
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
                            "version" => Ok(GeneratedField::Version),
                            "creationTimestamp" => Ok(GeneratedField::CreationTimestamp),
                            "tableName" => Ok(GeneratedField::TableName),
                            "partitionKey" => Ok(GeneratedField::PartitionKey),
                            "chunkId" => Ok(GeneratedField::ChunkId),
                            "partitionCheckpoint" => Ok(GeneratedField::PartitionCheckpoint),
                            "databaseCheckpoint" => Ok(GeneratedField::DatabaseCheckpoint),
                            "timeOfFirstWrite" => Ok(GeneratedField::TimeOfFirstWrite),
                            "timeOfLastWrite" => Ok(GeneratedField::TimeOfLastWrite),
                            "chunkOrder" => Ok(GeneratedField::ChunkOrder),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IoxMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.IoxMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> Result<IoxMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version = None;
                let mut creation_timestamp = None;
                let mut table_name = None;
                let mut partition_key = None;
                let mut chunk_id = None;
                let mut partition_checkpoint = None;
                let mut database_checkpoint = None;
                let mut time_of_first_write = None;
                let mut time_of_last_write = None;
                let mut chunk_order = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::CreationTimestamp => {
                            if creation_timestamp.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationTimestamp"));
                            }
                            creation_timestamp = Some(map.next_value()?);
                        }
                        GeneratedField::TableName => {
                            if table_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name = Some(map.next_value()?);
                        }
                        GeneratedField::PartitionKey => {
                            if partition_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionKey"));
                            }
                            partition_key = Some(map.next_value()?);
                        }
                        GeneratedField::ChunkId => {
                            if chunk_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunkId"));
                            }
                            chunk_id = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::PartitionCheckpoint => {
                            if partition_checkpoint.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionCheckpoint"));
                            }
                            partition_checkpoint = Some(map.next_value()?);
                        }
                        GeneratedField::DatabaseCheckpoint => {
                            if database_checkpoint.is_some() {
                                return Err(serde::de::Error::duplicate_field("databaseCheckpoint"));
                            }
                            database_checkpoint = Some(map.next_value()?);
                        }
                        GeneratedField::TimeOfFirstWrite => {
                            if time_of_first_write.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeOfFirstWrite"));
                            }
                            time_of_first_write = Some(map.next_value()?);
                        }
                        GeneratedField::TimeOfLastWrite => {
                            if time_of_last_write.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeOfLastWrite"));
                            }
                            time_of_last_write = Some(map.next_value()?);
                        }
                        GeneratedField::ChunkOrder => {
                            if chunk_order.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunkOrder"));
                            }
                            chunk_order = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(IoxMetadata {
                    version: version.unwrap_or_default(),
                    creation_timestamp,
                    table_name: table_name.unwrap_or_default(),
                    partition_key: partition_key.unwrap_or_default(),
                    chunk_id: chunk_id.unwrap_or_default(),
                    partition_checkpoint,
                    database_checkpoint,
                    time_of_first_write,
                    time_of_last_write,
                    chunk_order: chunk_order.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.IoxMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Op {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "OP_UNSPECIFIED",
            Self::Eq => "OP_EQ",
            Self::Ne => "OP_NE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Op {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OP_UNSPECIFIED",
            "OP_EQ",
            "OP_NE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Op;

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
                    .and_then(Op::from_i32)
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
                    .and_then(Op::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "OP_UNSPECIFIED" => Ok(Op::Unspecified),
                    "OP_EQ" => Ok(Op::Eq),
                    "OP_NE" => Ok(Op::Ne),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OptionalMinMaxSequence {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min.is_some() {
            len += 1;
        }
        if self.max != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.OptionalMinMaxSequence", len)?;
        if let Some(v) = self.min.as_ref() {
            struct_ser.serialize_field("min", v)?;
        }
        if self.max != 0 {
            struct_ser.serialize_field("max", ToString::to_string(&self.max).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OptionalMinMaxSequence {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min",
            "max",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Min,
            Max,
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
                            "min" => Ok(GeneratedField::Min),
                            "max" => Ok(GeneratedField::Max),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OptionalMinMaxSequence;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.OptionalMinMaxSequence")
            }

            fn visit_map<V>(self, mut map: V) -> Result<OptionalMinMaxSequence, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min = None;
                let mut max = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Min => {
                            if min.is_some() {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            min = Some(map.next_value()?);
                        }
                        GeneratedField::Max => {
                            if max.is_some() {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            max = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(OptionalMinMaxSequence {
                    min,
                    max: max.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.OptionalMinMaxSequence", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OptionalString {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.OptionalString", len)?;
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OptionalString {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
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
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OptionalString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.OptionalString")
            }

            fn visit_map<V>(self, mut map: V) -> Result<OptionalString, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                    }
                }
                Ok(OptionalString {
                    value: value.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.OptionalString", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OptionalStringSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.OptionalStringSet", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OptionalStringSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
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
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OptionalStringSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.OptionalStringSet")
            }

            fn visit_map<V>(self, mut map: V) -> Result<OptionalStringSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values = Some(map.next_value()?);
                        }
                    }
                }
                Ok(OptionalStringSet {
                    values: values.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.OptionalStringSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OptionalUint64 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.OptionalUint64", len)?;
        if self.value != 0 {
            struct_ser.serialize_field("value", ToString::to_string(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OptionalUint64 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
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
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OptionalUint64;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.OptionalUint64")
            }

            fn visit_map<V>(self, mut map: V) -> Result<OptionalUint64, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(OptionalUint64 {
                    value: value.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.OptionalUint64", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PartitionCheckpoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sequencer_numbers.is_empty() {
            len += 1;
        }
        if self.min_unpersisted_timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.PartitionCheckpoint", len)?;
        if !self.sequencer_numbers.is_empty() {
            struct_ser.serialize_field("sequencerNumbers", &self.sequencer_numbers)?;
        }
        if let Some(v) = self.min_unpersisted_timestamp.as_ref() {
            struct_ser.serialize_field("minUnpersistedTimestamp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PartitionCheckpoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sequencerNumbers",
            "minUnpersistedTimestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SequencerNumbers,
            MinUnpersistedTimestamp,
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
                            "sequencerNumbers" => Ok(GeneratedField::SequencerNumbers),
                            "minUnpersistedTimestamp" => Ok(GeneratedField::MinUnpersistedTimestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PartitionCheckpoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.PartitionCheckpoint")
            }

            fn visit_map<V>(self, mut map: V) -> Result<PartitionCheckpoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sequencer_numbers = None;
                let mut min_unpersisted_timestamp = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SequencerNumbers => {
                            if sequencer_numbers.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequencerNumbers"));
                            }
                            sequencer_numbers = Some(
                                map.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<u32>, _>>()?
                                    .into_iter().map(|(k,v)| (k.0, v)).collect()
                            );
                        }
                        GeneratedField::MinUnpersistedTimestamp => {
                            if min_unpersisted_timestamp.is_some() {
                                return Err(serde::de::Error::duplicate_field("minUnpersistedTimestamp"));
                            }
                            min_unpersisted_timestamp = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PartitionCheckpoint {
                    sequencer_numbers: sequencer_numbers.unwrap_or_default(),
                    min_unpersisted_timestamp,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.PartitionCheckpoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Path {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.directories.is_empty() {
            len += 1;
        }
        if !self.file_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.Path", len)?;
        if !self.directories.is_empty() {
            struct_ser.serialize_field("directories", &self.directories)?;
        }
        if !self.file_name.is_empty() {
            struct_ser.serialize_field("fileName", &self.file_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Path {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "directories",
            "fileName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Directories,
            FileName,
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
                            "directories" => Ok(GeneratedField::Directories),
                            "fileName" => Ok(GeneratedField::FileName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Path;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.Path")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Path, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut directories = None;
                let mut file_name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Directories => {
                            if directories.is_some() {
                                return Err(serde::de::Error::duplicate_field("directories"));
                            }
                            directories = Some(map.next_value()?);
                        }
                        GeneratedField::FileName => {
                            if file_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            file_name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Path {
                    directories: directories.unwrap_or_default(),
                    file_name: file_name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.Path", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Predicate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.table_names.is_some() {
            len += 1;
        }
        if self.field_columns.is_some() {
            len += 1;
        }
        if self.partition_key.is_some() {
            len += 1;
        }
        if self.range.is_some() {
            len += 1;
        }
        if !self.exprs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.Predicate", len)?;
        if let Some(v) = self.table_names.as_ref() {
            struct_ser.serialize_field("tableNames", v)?;
        }
        if let Some(v) = self.field_columns.as_ref() {
            struct_ser.serialize_field("fieldColumns", v)?;
        }
        if let Some(v) = self.partition_key.as_ref() {
            struct_ser.serialize_field("partitionKey", v)?;
        }
        if let Some(v) = self.range.as_ref() {
            struct_ser.serialize_field("range", v)?;
        }
        if !self.exprs.is_empty() {
            struct_ser.serialize_field("exprs", &self.exprs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Predicate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tableNames",
            "fieldColumns",
            "partitionKey",
            "range",
            "exprs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableNames,
            FieldColumns,
            PartitionKey,
            Range,
            Exprs,
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
                            "tableNames" => Ok(GeneratedField::TableNames),
                            "fieldColumns" => Ok(GeneratedField::FieldColumns),
                            "partitionKey" => Ok(GeneratedField::PartitionKey),
                            "range" => Ok(GeneratedField::Range),
                            "exprs" => Ok(GeneratedField::Exprs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Predicate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.Predicate")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Predicate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_names = None;
                let mut field_columns = None;
                let mut partition_key = None;
                let mut range = None;
                let mut exprs = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TableNames => {
                            if table_names.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableNames"));
                            }
                            table_names = Some(map.next_value()?);
                        }
                        GeneratedField::FieldColumns => {
                            if field_columns.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldColumns"));
                            }
                            field_columns = Some(map.next_value()?);
                        }
                        GeneratedField::PartitionKey => {
                            if partition_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionKey"));
                            }
                            partition_key = Some(map.next_value()?);
                        }
                        GeneratedField::Range => {
                            if range.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range = Some(map.next_value()?);
                        }
                        GeneratedField::Exprs => {
                            if exprs.is_some() {
                                return Err(serde::de::Error::duplicate_field("exprs"));
                            }
                            exprs = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Predicate {
                    table_names,
                    field_columns,
                    partition_key,
                    range,
                    exprs: exprs.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.Predicate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveParquet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.path.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.RemoveParquet", len)?;
        if let Some(v) = self.path.as_ref() {
            struct_ser.serialize_field("path", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveParquet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
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
                            "path" => Ok(GeneratedField::Path),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveParquet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.RemoveParquet")
            }

            fn visit_map<V>(self, mut map: V) -> Result<RemoveParquet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RemoveParquet {
                    path,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.RemoveParquet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Scalar {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.Scalar", len)?;
        if let Some(v) = self.value.as_ref() {
            match v {
                scalar::Value::ValueBool(v) => {
                    struct_ser.serialize_field("valueBool", v)?;
                }
                scalar::Value::ValueI64(v) => {
                    struct_ser.serialize_field("valueI64", ToString::to_string(&v).as_str())?;
                }
                scalar::Value::ValueF64(v) => {
                    struct_ser.serialize_field("valueF64", v)?;
                }
                scalar::Value::ValueString(v) => {
                    struct_ser.serialize_field("valueString", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Scalar {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "valueBool",
            "valueI64",
            "valueF64",
            "valueString",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValueBool,
            ValueI64,
            ValueF64,
            ValueString,
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
                            "valueBool" => Ok(GeneratedField::ValueBool),
                            "valueI64" => Ok(GeneratedField::ValueI64),
                            "valueF64" => Ok(GeneratedField::ValueF64),
                            "valueString" => Ok(GeneratedField::ValueString),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Scalar;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.Scalar")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Scalar, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ValueBool => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueBool"));
                            }
                            value = Some(scalar::Value::ValueBool(map.next_value()?));
                        }
                        GeneratedField::ValueI64 => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueI64"));
                            }
                            value = Some(scalar::Value::ValueI64(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            ));
                        }
                        GeneratedField::ValueF64 => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueF64"));
                            }
                            value = Some(scalar::Value::ValueF64(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            ));
                        }
                        GeneratedField::ValueString => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueString"));
                            }
                            value = Some(scalar::Value::ValueString(map.next_value()?));
                        }
                    }
                }
                Ok(Scalar {
                    value,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.Scalar", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimestampRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start != 0 {
            len += 1;
        }
        if self.end != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.TimestampRange", len)?;
        if self.start != 0 {
            struct_ser.serialize_field("start", ToString::to_string(&self.start).as_str())?;
        }
        if self.end != 0 {
            struct_ser.serialize_field("end", ToString::to_string(&self.end).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimestampRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start",
            "end",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            End,
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
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimestampRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.TimestampRange")
            }

            fn visit_map<V>(self, mut map: V) -> Result<TimestampRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start = None;
                let mut end = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if start.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::End => {
                            if end.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(TimestampRange {
                    start: start.unwrap_or_default(),
                    end: end.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.TimestampRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Transaction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        if self.revision_counter != 0 {
            len += 1;
        }
        if !self.uuid.is_empty() {
            len += 1;
        }
        if !self.previous_uuid.is_empty() {
            len += 1;
        }
        if self.start_timestamp.is_some() {
            len += 1;
        }
        if self.encoding != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.Transaction", len)?;
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        if self.revision_counter != 0 {
            struct_ser.serialize_field("revisionCounter", ToString::to_string(&self.revision_counter).as_str())?;
        }
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.previous_uuid.is_empty() {
            struct_ser.serialize_field("previousUuid", &self.previous_uuid)?;
        }
        if let Some(v) = self.start_timestamp.as_ref() {
            struct_ser.serialize_field("startTimestamp", v)?;
        }
        if self.encoding != 0 {
            let v = transaction::Encoding::from_i32(self.encoding)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.encoding)))?;
            struct_ser.serialize_field("encoding", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Transaction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "actions",
            "revisionCounter",
            "uuid",
            "previousUuid",
            "startTimestamp",
            "encoding",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Actions,
            RevisionCounter,
            Uuid,
            PreviousUuid,
            StartTimestamp,
            Encoding,
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
                            "version" => Ok(GeneratedField::Version),
                            "actions" => Ok(GeneratedField::Actions),
                            "revisionCounter" => Ok(GeneratedField::RevisionCounter),
                            "uuid" => Ok(GeneratedField::Uuid),
                            "previousUuid" => Ok(GeneratedField::PreviousUuid),
                            "startTimestamp" => Ok(GeneratedField::StartTimestamp),
                            "encoding" => Ok(GeneratedField::Encoding),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Transaction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.Transaction")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Transaction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version = None;
                let mut actions = None;
                let mut revision_counter = None;
                let mut uuid = None;
                let mut previous_uuid = None;
                let mut start_timestamp = None;
                let mut encoding = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Actions => {
                            if actions.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions = Some(map.next_value()?);
                        }
                        GeneratedField::RevisionCounter => {
                            if revision_counter.is_some() {
                                return Err(serde::de::Error::duplicate_field("revisionCounter"));
                            }
                            revision_counter = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Uuid => {
                            if uuid.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid = Some(map.next_value()?);
                        }
                        GeneratedField::PreviousUuid => {
                            if previous_uuid.is_some() {
                                return Err(serde::de::Error::duplicate_field("previousUuid"));
                            }
                            previous_uuid = Some(map.next_value()?);
                        }
                        GeneratedField::StartTimestamp => {
                            if start_timestamp.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTimestamp"));
                            }
                            start_timestamp = Some(map.next_value()?);
                        }
                        GeneratedField::Encoding => {
                            if encoding.is_some() {
                                return Err(serde::de::Error::duplicate_field("encoding"));
                            }
                            encoding = Some(map.next_value::<transaction::Encoding>()? as i32);
                        }
                    }
                }
                Ok(Transaction {
                    version: version.unwrap_or_default(),
                    actions: actions.unwrap_or_default(),
                    revision_counter: revision_counter.unwrap_or_default(),
                    uuid: uuid.unwrap_or_default(),
                    previous_uuid: previous_uuid.unwrap_or_default(),
                    start_timestamp,
                    encoding: encoding.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.Transaction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for transaction::Action {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.Transaction.Action", len)?;
        if let Some(v) = self.action.as_ref() {
            match v {
                transaction::action::Action::Upgrade(v) => {
                    struct_ser.serialize_field("upgrade", v)?;
                }
                transaction::action::Action::AddParquet(v) => {
                    struct_ser.serialize_field("addParquet", v)?;
                }
                transaction::action::Action::RemoveParquet(v) => {
                    struct_ser.serialize_field("removeParquet", v)?;
                }
                transaction::action::Action::DeletePredicate(v) => {
                    struct_ser.serialize_field("deletePredicate", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for transaction::Action {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "upgrade",
            "addParquet",
            "removeParquet",
            "deletePredicate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Upgrade,
            AddParquet,
            RemoveParquet,
            DeletePredicate,
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
                            "upgrade" => Ok(GeneratedField::Upgrade),
                            "addParquet" => Ok(GeneratedField::AddParquet),
                            "removeParquet" => Ok(GeneratedField::RemoveParquet),
                            "deletePredicate" => Ok(GeneratedField::DeletePredicate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = transaction::Action;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.Transaction.Action")
            }

            fn visit_map<V>(self, mut map: V) -> Result<transaction::Action, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Upgrade => {
                            if action.is_some() {
                                return Err(serde::de::Error::duplicate_field("upgrade"));
                            }
                            action = Some(transaction::action::Action::Upgrade(map.next_value()?));
                        }
                        GeneratedField::AddParquet => {
                            if action.is_some() {
                                return Err(serde::de::Error::duplicate_field("addParquet"));
                            }
                            action = Some(transaction::action::Action::AddParquet(map.next_value()?));
                        }
                        GeneratedField::RemoveParquet => {
                            if action.is_some() {
                                return Err(serde::de::Error::duplicate_field("removeParquet"));
                            }
                            action = Some(transaction::action::Action::RemoveParquet(map.next_value()?));
                        }
                        GeneratedField::DeletePredicate => {
                            if action.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletePredicate"));
                            }
                            action = Some(transaction::action::Action::DeletePredicate(map.next_value()?));
                        }
                    }
                }
                Ok(transaction::Action {
                    action,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.Transaction.Action", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for transaction::Encoding {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ENCODING_UNSPECIFIED",
            Self::Delta => "ENCODING_DELTA",
            Self::Full => "ENCODING_FULL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for transaction::Encoding {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ENCODING_UNSPECIFIED",
            "ENCODING_DELTA",
            "ENCODING_FULL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = transaction::Encoding;

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
                    .and_then(transaction::Encoding::from_i32)
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
                    .and_then(transaction::Encoding::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ENCODING_UNSPECIFIED" => Ok(transaction::Encoding::Unspecified),
                    "ENCODING_DELTA" => Ok(transaction::Encoding::Delta),
                    "ENCODING_FULL" => Ok(transaction::Encoding::Full),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Upgrade {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.format.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.catalog.v1.Upgrade", len)?;
        if !self.format.is_empty() {
            struct_ser.serialize_field("format", &self.format)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Upgrade {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "format",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Format,
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
                            "format" => Ok(GeneratedField::Format),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Upgrade;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.catalog.v1.Upgrade")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Upgrade, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut format = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Format => {
                            if format.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            format = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Upgrade {
                    format: format.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.catalog.v1.Upgrade", FIELDS, GeneratedVisitor)
    }
}
