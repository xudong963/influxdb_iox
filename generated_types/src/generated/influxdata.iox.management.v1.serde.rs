impl serde::Serialize for Chunk {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.partition_key.is_empty() {
            len += 1;
        }
        if !self.table_name.is_empty() {
            len += 1;
        }
        if self.id != 0 {
            len += 1;
        }
        if self.storage != 0 {
            len += 1;
        }
        if self.lifecycle_action != 0 {
            len += 1;
        }
        if self.memory_bytes != 0 {
            len += 1;
        }
        if self.object_store_bytes != 0 {
            len += 1;
        }
        if self.row_count != 0 {
            len += 1;
        }
        if self.time_of_last_access.is_some() {
            len += 1;
        }
        if self.time_of_first_write.is_some() {
            len += 1;
        }
        if self.time_of_last_write.is_some() {
            len += 1;
        }
        if self.time_closed.is_some() {
            len += 1;
        }
        if self.order != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.Chunk", len)?;
        if !self.partition_key.is_empty() {
            struct_ser.serialize_field("partitionKey", &self.partition_key)?;
        }
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.storage != 0 {
            let v = ChunkStorage::from_i32(self.storage)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.storage)))?;
            struct_ser.serialize_field("storage", &v)?;
        }
        if self.lifecycle_action != 0 {
            let v = ChunkLifecycleAction::from_i32(self.lifecycle_action)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.lifecycle_action)))?;
            struct_ser.serialize_field("lifecycleAction", &v)?;
        }
        if self.memory_bytes != 0 {
            struct_ser.serialize_field("memoryBytes", ToString::to_string(&self.memory_bytes).as_str())?;
        }
        if self.object_store_bytes != 0 {
            struct_ser.serialize_field("objectStoreBytes", ToString::to_string(&self.object_store_bytes).as_str())?;
        }
        if self.row_count != 0 {
            struct_ser.serialize_field("rowCount", ToString::to_string(&self.row_count).as_str())?;
        }
        if let Some(v) = self.time_of_last_access.as_ref() {
            struct_ser.serialize_field("timeOfLastAccess", v)?;
        }
        if let Some(v) = self.time_of_first_write.as_ref() {
            struct_ser.serialize_field("timeOfFirstWrite", v)?;
        }
        if let Some(v) = self.time_of_last_write.as_ref() {
            struct_ser.serialize_field("timeOfLastWrite", v)?;
        }
        if let Some(v) = self.time_closed.as_ref() {
            struct_ser.serialize_field("timeClosed", v)?;
        }
        if self.order != 0 {
            struct_ser.serialize_field("order", &self.order)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Chunk {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "partitionKey",
            "tableName",
            "id",
            "storage",
            "lifecycleAction",
            "memoryBytes",
            "objectStoreBytes",
            "rowCount",
            "timeOfLastAccess",
            "timeOfFirstWrite",
            "timeOfLastWrite",
            "timeClosed",
            "order",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PartitionKey,
            TableName,
            Id,
            Storage,
            LifecycleAction,
            MemoryBytes,
            ObjectStoreBytes,
            RowCount,
            TimeOfLastAccess,
            TimeOfFirstWrite,
            TimeOfLastWrite,
            TimeClosed,
            Order,
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
                            "partitionKey" => Ok(GeneratedField::PartitionKey),
                            "tableName" => Ok(GeneratedField::TableName),
                            "id" => Ok(GeneratedField::Id),
                            "storage" => Ok(GeneratedField::Storage),
                            "lifecycleAction" => Ok(GeneratedField::LifecycleAction),
                            "memoryBytes" => Ok(GeneratedField::MemoryBytes),
                            "objectStoreBytes" => Ok(GeneratedField::ObjectStoreBytes),
                            "rowCount" => Ok(GeneratedField::RowCount),
                            "timeOfLastAccess" => Ok(GeneratedField::TimeOfLastAccess),
                            "timeOfFirstWrite" => Ok(GeneratedField::TimeOfFirstWrite),
                            "timeOfLastWrite" => Ok(GeneratedField::TimeOfLastWrite),
                            "timeClosed" => Ok(GeneratedField::TimeClosed),
                            "order" => Ok(GeneratedField::Order),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Chunk;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.Chunk")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Chunk, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut partition_key = None;
                let mut table_name = None;
                let mut id = None;
                let mut storage = None;
                let mut lifecycle_action = None;
                let mut memory_bytes = None;
                let mut object_store_bytes = None;
                let mut row_count = None;
                let mut time_of_last_access = None;
                let mut time_of_first_write = None;
                let mut time_of_last_write = None;
                let mut time_closed = None;
                let mut order = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PartitionKey => {
                            if partition_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionKey"));
                            }
                            partition_key = Some(map.next_value()?);
                        }
                        GeneratedField::TableName => {
                            if table_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name = Some(map.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Storage => {
                            if storage.is_some() {
                                return Err(serde::de::Error::duplicate_field("storage"));
                            }
                            storage = Some(map.next_value::<ChunkStorage>()? as i32);
                        }
                        GeneratedField::LifecycleAction => {
                            if lifecycle_action.is_some() {
                                return Err(serde::de::Error::duplicate_field("lifecycleAction"));
                            }
                            lifecycle_action = Some(map.next_value::<ChunkLifecycleAction>()? as i32);
                        }
                        GeneratedField::MemoryBytes => {
                            if memory_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("memoryBytes"));
                            }
                            memory_bytes = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::ObjectStoreBytes => {
                            if object_store_bytes.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectStoreBytes"));
                            }
                            object_store_bytes = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::RowCount => {
                            if row_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("rowCount"));
                            }
                            row_count = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::TimeOfLastAccess => {
                            if time_of_last_access.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeOfLastAccess"));
                            }
                            time_of_last_access = Some(map.next_value()?);
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
                        GeneratedField::TimeClosed => {
                            if time_closed.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeClosed"));
                            }
                            time_closed = Some(map.next_value()?);
                        }
                        GeneratedField::Order => {
                            if order.is_some() {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            order = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(Chunk {
                    partition_key: partition_key.unwrap_or_default(),
                    table_name: table_name.unwrap_or_default(),
                    id: id.unwrap_or_default(),
                    storage: storage.unwrap_or_default(),
                    lifecycle_action: lifecycle_action.unwrap_or_default(),
                    memory_bytes: memory_bytes.unwrap_or_default(),
                    object_store_bytes: object_store_bytes.unwrap_or_default(),
                    row_count: row_count.unwrap_or_default(),
                    time_of_last_access,
                    time_of_first_write,
                    time_of_last_write,
                    time_closed,
                    order: order.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.Chunk", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChunkLifecycleAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CHUNK_LIFECYCLE_ACTION_UNSPECIFIED",
            Self::Persisting => "CHUNK_LIFECYCLE_ACTION_PERSISTING",
            Self::Compacting => "CHUNK_LIFECYCLE_ACTION_COMPACTING",
            Self::Dropping => "CHUNK_LIFECYCLE_ACTION_DROPPING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ChunkLifecycleAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CHUNK_LIFECYCLE_ACTION_UNSPECIFIED",
            "CHUNK_LIFECYCLE_ACTION_PERSISTING",
            "CHUNK_LIFECYCLE_ACTION_COMPACTING",
            "CHUNK_LIFECYCLE_ACTION_DROPPING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChunkLifecycleAction;

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
                    .and_then(ChunkLifecycleAction::from_i32)
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
                    .and_then(ChunkLifecycleAction::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CHUNK_LIFECYCLE_ACTION_UNSPECIFIED" => Ok(ChunkLifecycleAction::Unspecified),
                    "CHUNK_LIFECYCLE_ACTION_PERSISTING" => Ok(ChunkLifecycleAction::Persisting),
                    "CHUNK_LIFECYCLE_ACTION_COMPACTING" => Ok(ChunkLifecycleAction::Compacting),
                    "CHUNK_LIFECYCLE_ACTION_DROPPING" => Ok(ChunkLifecycleAction::Dropping),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ChunkStorage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CHUNK_STORAGE_UNSPECIFIED",
            Self::OpenMutableBuffer => "CHUNK_STORAGE_OPEN_MUTABLE_BUFFER",
            Self::ClosedMutableBuffer => "CHUNK_STORAGE_CLOSED_MUTABLE_BUFFER",
            Self::ReadBuffer => "CHUNK_STORAGE_READ_BUFFER",
            Self::ReadBufferAndObjectStore => "CHUNK_STORAGE_READ_BUFFER_AND_OBJECT_STORE",
            Self::ObjectStoreOnly => "CHUNK_STORAGE_OBJECT_STORE_ONLY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ChunkStorage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CHUNK_STORAGE_UNSPECIFIED",
            "CHUNK_STORAGE_OPEN_MUTABLE_BUFFER",
            "CHUNK_STORAGE_CLOSED_MUTABLE_BUFFER",
            "CHUNK_STORAGE_READ_BUFFER",
            "CHUNK_STORAGE_READ_BUFFER_AND_OBJECT_STORE",
            "CHUNK_STORAGE_OBJECT_STORE_ONLY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChunkStorage;

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
                    .and_then(ChunkStorage::from_i32)
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
                    .and_then(ChunkStorage::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CHUNK_STORAGE_UNSPECIFIED" => Ok(ChunkStorage::Unspecified),
                    "CHUNK_STORAGE_OPEN_MUTABLE_BUFFER" => Ok(ChunkStorage::OpenMutableBuffer),
                    "CHUNK_STORAGE_CLOSED_MUTABLE_BUFFER" => Ok(ChunkStorage::ClosedMutableBuffer),
                    "CHUNK_STORAGE_READ_BUFFER" => Ok(ChunkStorage::ReadBuffer),
                    "CHUNK_STORAGE_READ_BUFFER_AND_OBJECT_STORE" => Ok(ChunkStorage::ReadBufferAndObjectStore),
                    "CHUNK_STORAGE_OBJECT_STORE_ONLY" => Ok(ChunkStorage::ObjectStoreOnly),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ClosePartitionChunkRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        if !self.partition_key.is_empty() {
            len += 1;
        }
        if !self.table_name.is_empty() {
            len += 1;
        }
        if self.chunk_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ClosePartitionChunkRequest", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        if !self.partition_key.is_empty() {
            struct_ser.serialize_field("partitionKey", &self.partition_key)?;
        }
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if self.chunk_id != 0 {
            struct_ser.serialize_field("chunkId", &self.chunk_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClosePartitionChunkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
            "partitionKey",
            "tableName",
            "chunkId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
            PartitionKey,
            TableName,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            "partitionKey" => Ok(GeneratedField::PartitionKey),
                            "tableName" => Ok(GeneratedField::TableName),
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
            type Value = ClosePartitionChunkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ClosePartitionChunkRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ClosePartitionChunkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                let mut partition_key = None;
                let mut table_name = None;
                let mut chunk_id = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                        GeneratedField::PartitionKey => {
                            if partition_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionKey"));
                            }
                            partition_key = Some(map.next_value()?);
                        }
                        GeneratedField::TableName => {
                            if table_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name = Some(map.next_value()?);
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
                Ok(ClosePartitionChunkRequest {
                    db_name: db_name.unwrap_or_default(),
                    partition_key: partition_key.unwrap_or_default(),
                    table_name: table_name.unwrap_or_default(),
                    chunk_id: chunk_id.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ClosePartitionChunkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClosePartitionChunkResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.operation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ClosePartitionChunkResponse", len)?;
        if let Some(v) = self.operation.as_ref() {
            struct_ser.serialize_field("operation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClosePartitionChunkResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operation,
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
                            "operation" => Ok(GeneratedField::Operation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClosePartitionChunkResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ClosePartitionChunkResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ClosePartitionChunkResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operation = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Operation => {
                            if operation.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClosePartitionChunkResponse {
                    operation,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ClosePartitionChunkResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CompactChunks {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        if !self.partition_key.is_empty() {
            len += 1;
        }
        if !self.table_name.is_empty() {
            len += 1;
        }
        if !self.chunks.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.CompactChunks", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        if !self.partition_key.is_empty() {
            struct_ser.serialize_field("partitionKey", &self.partition_key)?;
        }
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if !self.chunks.is_empty() {
            struct_ser.serialize_field("chunks", &self.chunks)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CompactChunks {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
            "partitionKey",
            "tableName",
            "chunks",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
            PartitionKey,
            TableName,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            "partitionKey" => Ok(GeneratedField::PartitionKey),
                            "tableName" => Ok(GeneratedField::TableName),
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
            type Value = CompactChunks;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.CompactChunks")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CompactChunks, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                let mut partition_key = None;
                let mut table_name = None;
                let mut chunks = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                        GeneratedField::PartitionKey => {
                            if partition_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionKey"));
                            }
                            partition_key = Some(map.next_value()?);
                        }
                        GeneratedField::TableName => {
                            if table_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name = Some(map.next_value()?);
                        }
                        GeneratedField::Chunks => {
                            if chunks.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunks"));
                            }
                            chunks = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                    }
                }
                Ok(CompactChunks {
                    db_name: db_name.unwrap_or_default(),
                    partition_key: partition_key.unwrap_or_default(),
                    table_name: table_name.unwrap_or_default(),
                    chunks: chunks.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.CompactChunks", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateDatabaseRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rules.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.CreateDatabaseRequest", len)?;
        if let Some(v) = self.rules.as_ref() {
            struct_ser.serialize_field("rules", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateDatabaseRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
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
                            "rules" => Ok(GeneratedField::Rules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateDatabaseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.CreateDatabaseRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CreateDatabaseRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CreateDatabaseRequest {
                    rules,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.CreateDatabaseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateDatabaseResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.CreateDatabaseResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateDatabaseResponse {
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
            type Value = CreateDatabaseResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.CreateDatabaseResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CreateDatabaseResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(CreateDatabaseResponse {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.CreateDatabaseResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateDummyJobRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.nanos.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.CreateDummyJobRequest", len)?;
        if !self.nanos.is_empty() {
            struct_ser.serialize_field("nanos", &self.nanos.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateDummyJobRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nanos",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nanos,
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
                            "nanos" => Ok(GeneratedField::Nanos),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateDummyJobRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.CreateDummyJobRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CreateDummyJobRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut nanos = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Nanos => {
                            if nanos.is_some() {
                                return Err(serde::de::Error::duplicate_field("nanos"));
                            }
                            nanos = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                    }
                }
                Ok(CreateDummyJobRequest {
                    nanos: nanos.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.CreateDummyJobRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateDummyJobResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.operation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.CreateDummyJobResponse", len)?;
        if let Some(v) = self.operation.as_ref() {
            struct_ser.serialize_field("operation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateDummyJobResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operation,
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
                            "operation" => Ok(GeneratedField::Operation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateDummyJobResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.CreateDummyJobResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CreateDummyJobResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operation = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Operation => {
                            if operation.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CreateDummyJobResponse {
                    operation,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.CreateDummyJobResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DatabaseRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.partition_template.is_some() {
            len += 1;
        }
        if self.lifecycle_rules.is_some() {
            len += 1;
        }
        if self.worker_cleanup_avg_sleep.is_some() {
            len += 1;
        }
        if self.write_buffer_connection.is_some() {
            len += 1;
        }
        if self.routing_rules.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.DatabaseRules", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.partition_template.as_ref() {
            struct_ser.serialize_field("partitionTemplate", v)?;
        }
        if let Some(v) = self.lifecycle_rules.as_ref() {
            struct_ser.serialize_field("lifecycleRules", v)?;
        }
        if let Some(v) = self.worker_cleanup_avg_sleep.as_ref() {
            struct_ser.serialize_field("workerCleanupAvgSleep", v)?;
        }
        if let Some(v) = self.write_buffer_connection.as_ref() {
            struct_ser.serialize_field("writeBufferConnection", v)?;
        }
        if let Some(v) = self.routing_rules.as_ref() {
            match v {
                database_rules::RoutingRules::ShardConfig(v) => {
                    struct_ser.serialize_field("shardConfig", v)?;
                }
                database_rules::RoutingRules::RoutingConfig(v) => {
                    struct_ser.serialize_field("routingConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DatabaseRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "partitionTemplate",
            "lifecycleRules",
            "workerCleanupAvgSleep",
            "writeBufferConnection",
            "shardConfig",
            "routingConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            PartitionTemplate,
            LifecycleRules,
            WorkerCleanupAvgSleep,
            WriteBufferConnection,
            ShardConfig,
            RoutingConfig,
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
                            "name" => Ok(GeneratedField::Name),
                            "partitionTemplate" => Ok(GeneratedField::PartitionTemplate),
                            "lifecycleRules" => Ok(GeneratedField::LifecycleRules),
                            "workerCleanupAvgSleep" => Ok(GeneratedField::WorkerCleanupAvgSleep),
                            "writeBufferConnection" => Ok(GeneratedField::WriteBufferConnection),
                            "shardConfig" => Ok(GeneratedField::ShardConfig),
                            "routingConfig" => Ok(GeneratedField::RoutingConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DatabaseRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.DatabaseRules")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DatabaseRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name = None;
                let mut partition_template = None;
                let mut lifecycle_rules = None;
                let mut worker_cleanup_avg_sleep = None;
                let mut write_buffer_connection = None;
                let mut routing_rules = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        GeneratedField::PartitionTemplate => {
                            if partition_template.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionTemplate"));
                            }
                            partition_template = Some(map.next_value()?);
                        }
                        GeneratedField::LifecycleRules => {
                            if lifecycle_rules.is_some() {
                                return Err(serde::de::Error::duplicate_field("lifecycleRules"));
                            }
                            lifecycle_rules = Some(map.next_value()?);
                        }
                        GeneratedField::WorkerCleanupAvgSleep => {
                            if worker_cleanup_avg_sleep.is_some() {
                                return Err(serde::de::Error::duplicate_field("workerCleanupAvgSleep"));
                            }
                            worker_cleanup_avg_sleep = Some(map.next_value()?);
                        }
                        GeneratedField::WriteBufferConnection => {
                            if write_buffer_connection.is_some() {
                                return Err(serde::de::Error::duplicate_field("writeBufferConnection"));
                            }
                            write_buffer_connection = Some(map.next_value()?);
                        }
                        GeneratedField::ShardConfig => {
                            if routing_rules.is_some() {
                                return Err(serde::de::Error::duplicate_field("shardConfig"));
                            }
                            routing_rules = Some(database_rules::RoutingRules::ShardConfig(map.next_value()?));
                        }
                        GeneratedField::RoutingConfig => {
                            if routing_rules.is_some() {
                                return Err(serde::de::Error::duplicate_field("routingConfig"));
                            }
                            routing_rules = Some(database_rules::RoutingRules::RoutingConfig(map.next_value()?));
                        }
                    }
                }
                Ok(DatabaseRules {
                    name: name.unwrap_or_default(),
                    partition_template,
                    lifecycle_rules,
                    worker_cleanup_avg_sleep,
                    write_buffer_connection,
                    routing_rules,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.DatabaseRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DatabaseStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.DatabaseStatus", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if self.state != 0 {
            let v = database_status::DatabaseState::from_i32(self.state)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DatabaseStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
            "error",
            "state",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
            Error,
            State,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            "error" => Ok(GeneratedField::Error),
                            "state" => Ok(GeneratedField::State),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DatabaseStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.DatabaseStatus")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DatabaseStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                let mut error = None;
                let mut state = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                        GeneratedField::Error => {
                            if error.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error = Some(map.next_value()?);
                        }
                        GeneratedField::State => {
                            if state.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state = Some(map.next_value::<database_status::DatabaseState>()? as i32);
                        }
                    }
                }
                Ok(DatabaseStatus {
                    db_name: db_name.unwrap_or_default(),
                    error,
                    state: state.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.DatabaseStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for database_status::DatabaseState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "DATABASE_STATE_UNSPECIFIED",
            Self::Known => "DATABASE_STATE_KNOWN",
            Self::DatabaseObjectStoreFound => "DATABASE_STATE_DATABASE_OBJECT_STORE_FOUND",
            Self::NoActiveDatabase => "DATABASE_STATE_NO_ACTIVE_DATABASE",
            Self::RulesLoaded => "DATABASE_STATE_RULES_LOADED",
            Self::CatalogLoaded => "DATABASE_STATE_CATALOG_LOADED",
            Self::Initialized => "DATABASE_STATE_INITIALIZED",
            Self::RulesLoadError => "DATABASE_STATE_RULES_LOAD_ERROR",
            Self::CatalogLoadError => "DATABASE_STATE_CATALOG_LOAD_ERROR",
            Self::ReplayError => "DATABASE_STATE_REPLAY_ERROR",
            Self::DatabaseObjectStoreLookupError => "DATABASE_STATE_DATABASE_OBJECT_STORE_LOOKUP_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for database_status::DatabaseState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DATABASE_STATE_UNSPECIFIED",
            "DATABASE_STATE_KNOWN",
            "DATABASE_STATE_DATABASE_OBJECT_STORE_FOUND",
            "DATABASE_STATE_NO_ACTIVE_DATABASE",
            "DATABASE_STATE_RULES_LOADED",
            "DATABASE_STATE_CATALOG_LOADED",
            "DATABASE_STATE_INITIALIZED",
            "DATABASE_STATE_RULES_LOAD_ERROR",
            "DATABASE_STATE_CATALOG_LOAD_ERROR",
            "DATABASE_STATE_REPLAY_ERROR",
            "DATABASE_STATE_DATABASE_OBJECT_STORE_LOOKUP_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = database_status::DatabaseState;

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
                    .and_then(database_status::DatabaseState::from_i32)
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
                    .and_then(database_status::DatabaseState::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DATABASE_STATE_UNSPECIFIED" => Ok(database_status::DatabaseState::Unspecified),
                    "DATABASE_STATE_KNOWN" => Ok(database_status::DatabaseState::Known),
                    "DATABASE_STATE_DATABASE_OBJECT_STORE_FOUND" => Ok(database_status::DatabaseState::DatabaseObjectStoreFound),
                    "DATABASE_STATE_NO_ACTIVE_DATABASE" => Ok(database_status::DatabaseState::NoActiveDatabase),
                    "DATABASE_STATE_RULES_LOADED" => Ok(database_status::DatabaseState::RulesLoaded),
                    "DATABASE_STATE_CATALOG_LOADED" => Ok(database_status::DatabaseState::CatalogLoaded),
                    "DATABASE_STATE_INITIALIZED" => Ok(database_status::DatabaseState::Initialized),
                    "DATABASE_STATE_RULES_LOAD_ERROR" => Ok(database_status::DatabaseState::RulesLoadError),
                    "DATABASE_STATE_CATALOG_LOAD_ERROR" => Ok(database_status::DatabaseState::CatalogLoadError),
                    "DATABASE_STATE_REPLAY_ERROR" => Ok(database_status::DatabaseState::ReplayError),
                    "DATABASE_STATE_DATABASE_OBJECT_STORE_LOOKUP_ERROR" => Ok(database_status::DatabaseState::DatabaseObjectStoreLookupError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Delete {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        if !self.table_name.is_empty() {
            len += 1;
        }
        if !self.delete_predicate.is_empty() {
            len += 1;
        }
        if !self.start_time.is_empty() {
            len += 1;
        }
        if !self.stop_time.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.Delete", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if !self.delete_predicate.is_empty() {
            struct_ser.serialize_field("deletePredicate", &self.delete_predicate)?;
        }
        if !self.start_time.is_empty() {
            struct_ser.serialize_field("startTime", &self.start_time)?;
        }
        if !self.stop_time.is_empty() {
            struct_ser.serialize_field("stopTime", &self.stop_time)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Delete {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
            "tableName",
            "deletePredicate",
            "startTime",
            "stopTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
            TableName,
            DeletePredicate,
            StartTime,
            StopTime,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            "tableName" => Ok(GeneratedField::TableName),
                            "deletePredicate" => Ok(GeneratedField::DeletePredicate),
                            "startTime" => Ok(GeneratedField::StartTime),
                            "stopTime" => Ok(GeneratedField::StopTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Delete;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.Delete")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Delete, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                let mut table_name = None;
                let mut delete_predicate = None;
                let mut start_time = None;
                let mut stop_time = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                        GeneratedField::TableName => {
                            if table_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name = Some(map.next_value()?);
                        }
                        GeneratedField::DeletePredicate => {
                            if delete_predicate.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletePredicate"));
                            }
                            delete_predicate = Some(map.next_value()?);
                        }
                        GeneratedField::StartTime => {
                            if start_time.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time = Some(map.next_value()?);
                        }
                        GeneratedField::StopTime => {
                            if stop_time.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopTime"));
                            }
                            stop_time = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Delete {
                    db_name: db_name.unwrap_or_default(),
                    table_name: table_name.unwrap_or_default(),
                    delete_predicate: delete_predicate.unwrap_or_default(),
                    start_time: start_time.unwrap_or_default(),
                    stop_time: stop_time.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.Delete", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteDatabaseRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.DeleteDatabaseRequest", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteDatabaseRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteDatabaseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.DeleteDatabaseRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DeleteDatabaseRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DeleteDatabaseRequest {
                    db_name: db_name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.DeleteDatabaseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteDatabaseResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.DeleteDatabaseResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteDatabaseResponse {
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
            type Value = DeleteDatabaseResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.DeleteDatabaseResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DeleteDatabaseResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(DeleteDatabaseResponse {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.DeleteDatabaseResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRemoteRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.DeleteRemoteRequest", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRemoteRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRemoteRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.DeleteRemoteRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DeleteRemoteRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(DeleteRemoteRequest {
                    id: id.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.DeleteRemoteRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRemoteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.DeleteRemoteResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRemoteResponse {
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
            type Value = DeleteRemoteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.DeleteRemoteResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DeleteRemoteResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(DeleteRemoteResponse {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.DeleteRemoteResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        if !self.table_name.is_empty() {
            len += 1;
        }
        if !self.start_time.is_empty() {
            len += 1;
        }
        if !self.stop_time.is_empty() {
            len += 1;
        }
        if !self.predicate.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.DeleteRequest", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if !self.start_time.is_empty() {
            struct_ser.serialize_field("startTime", &self.start_time)?;
        }
        if !self.stop_time.is_empty() {
            struct_ser.serialize_field("stopTime", &self.stop_time)?;
        }
        if !self.predicate.is_empty() {
            struct_ser.serialize_field("predicate", &self.predicate)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
            "tableName",
            "startTime",
            "stopTime",
            "predicate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
            TableName,
            StartTime,
            StopTime,
            Predicate,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            "tableName" => Ok(GeneratedField::TableName),
                            "startTime" => Ok(GeneratedField::StartTime),
                            "stopTime" => Ok(GeneratedField::StopTime),
                            "predicate" => Ok(GeneratedField::Predicate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.DeleteRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DeleteRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                let mut table_name = None;
                let mut start_time = None;
                let mut stop_time = None;
                let mut predicate = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                        GeneratedField::TableName => {
                            if table_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name = Some(map.next_value()?);
                        }
                        GeneratedField::StartTime => {
                            if start_time.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time = Some(map.next_value()?);
                        }
                        GeneratedField::StopTime => {
                            if stop_time.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopTime"));
                            }
                            stop_time = Some(map.next_value()?);
                        }
                        GeneratedField::Predicate => {
                            if predicate.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DeleteRequest {
                    db_name: db_name.unwrap_or_default(),
                    table_name: table_name.unwrap_or_default(),
                    start_time: start_time.unwrap_or_default(),
                    stop_time: stop_time.unwrap_or_default(),
                    predicate: predicate.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.DeleteRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.DeleteResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteResponse {
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
            type Value = DeleteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.DeleteResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DeleteResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(DeleteResponse {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.DeleteResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DetailedDatabase {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.generation_id != 0 {
            len += 1;
        }
        if self.deleted_at.is_some() {
            len += 1;
        }
        if !self.db_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.DetailedDatabase", len)?;
        if self.generation_id != 0 {
            struct_ser.serialize_field("generationId", ToString::to_string(&self.generation_id).as_str())?;
        }
        if let Some(v) = self.deleted_at.as_ref() {
            struct_ser.serialize_field("deletedAt", v)?;
        }
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DetailedDatabase {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "generationId",
            "deletedAt",
            "dbName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GenerationId,
            DeletedAt,
            DbName,
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
                            "generationId" => Ok(GeneratedField::GenerationId),
                            "deletedAt" => Ok(GeneratedField::DeletedAt),
                            "dbName" => Ok(GeneratedField::DbName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DetailedDatabase;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.DetailedDatabase")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DetailedDatabase, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut generation_id = None;
                let mut deleted_at = None;
                let mut db_name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GenerationId => {
                            if generation_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("generationId"));
                            }
                            generation_id = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::DeletedAt => {
                            if deleted_at.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletedAt"));
                            }
                            deleted_at = Some(map.next_value()?);
                        }
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DetailedDatabase {
                    generation_id: generation_id.unwrap_or_default(),
                    deleted_at,
                    db_name: db_name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.DetailedDatabase", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DevNull {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.DevNull", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DevNull {
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
            type Value = DevNull;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.DevNull")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DevNull, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(DevNull {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.DevNull", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DropChunk {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        if !self.partition_key.is_empty() {
            len += 1;
        }
        if !self.table_name.is_empty() {
            len += 1;
        }
        if self.chunk_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.DropChunk", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        if !self.partition_key.is_empty() {
            struct_ser.serialize_field("partitionKey", &self.partition_key)?;
        }
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if self.chunk_id != 0 {
            struct_ser.serialize_field("chunkId", &self.chunk_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DropChunk {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
            "partitionKey",
            "tableName",
            "chunkId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
            PartitionKey,
            TableName,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            "partitionKey" => Ok(GeneratedField::PartitionKey),
                            "tableName" => Ok(GeneratedField::TableName),
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
            type Value = DropChunk;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.DropChunk")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DropChunk, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                let mut partition_key = None;
                let mut table_name = None;
                let mut chunk_id = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                        GeneratedField::PartitionKey => {
                            if partition_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionKey"));
                            }
                            partition_key = Some(map.next_value()?);
                        }
                        GeneratedField::TableName => {
                            if table_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name = Some(map.next_value()?);
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
                Ok(DropChunk {
                    db_name: db_name.unwrap_or_default(),
                    partition_key: partition_key.unwrap_or_default(),
                    table_name: table_name.unwrap_or_default(),
                    chunk_id: chunk_id.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.DropChunk", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DropPartition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        if !self.partition_key.is_empty() {
            len += 1;
        }
        if !self.table_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.DropPartition", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        if !self.partition_key.is_empty() {
            struct_ser.serialize_field("partitionKey", &self.partition_key)?;
        }
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DropPartition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
            "partitionKey",
            "tableName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
            PartitionKey,
            TableName,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            "partitionKey" => Ok(GeneratedField::PartitionKey),
                            "tableName" => Ok(GeneratedField::TableName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DropPartition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.DropPartition")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DropPartition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                let mut partition_key = None;
                let mut table_name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                        GeneratedField::PartitionKey => {
                            if partition_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionKey"));
                            }
                            partition_key = Some(map.next_value()?);
                        }
                        GeneratedField::TableName => {
                            if table_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DropPartition {
                    db_name: db_name.unwrap_or_default(),
                    partition_key: partition_key.unwrap_or_default(),
                    table_name: table_name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.DropPartition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DropPartitionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        if !self.partition_key.is_empty() {
            len += 1;
        }
        if !self.table_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.DropPartitionRequest", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        if !self.partition_key.is_empty() {
            struct_ser.serialize_field("partitionKey", &self.partition_key)?;
        }
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DropPartitionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
            "partitionKey",
            "tableName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
            PartitionKey,
            TableName,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            "partitionKey" => Ok(GeneratedField::PartitionKey),
                            "tableName" => Ok(GeneratedField::TableName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DropPartitionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.DropPartitionRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DropPartitionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                let mut partition_key = None;
                let mut table_name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                        GeneratedField::PartitionKey => {
                            if partition_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionKey"));
                            }
                            partition_key = Some(map.next_value()?);
                        }
                        GeneratedField::TableName => {
                            if table_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DropPartitionRequest {
                    db_name: db_name.unwrap_or_default(),
                    partition_key: partition_key.unwrap_or_default(),
                    table_name: table_name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.DropPartitionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DropPartitionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.DropPartitionResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DropPartitionResponse {
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
            type Value = DropPartitionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.DropPartitionResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DropPartitionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(DropPartitionResponse {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.DropPartitionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Dummy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.nanos.is_empty() {
            len += 1;
        }
        if !self.db_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.Dummy", len)?;
        if !self.nanos.is_empty() {
            struct_ser.serialize_field("nanos", &self.nanos.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dummy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nanos",
            "dbName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nanos,
            DbName,
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
                            "nanos" => Ok(GeneratedField::Nanos),
                            "dbName" => Ok(GeneratedField::DbName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dummy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.Dummy")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Dummy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut nanos = None;
                let mut db_name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Nanos => {
                            if nanos.is_some() {
                                return Err(serde::de::Error::duplicate_field("nanos"));
                            }
                            nanos = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Dummy {
                    nanos: nanos.unwrap_or_default(),
                    db_name: db_name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.Dummy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Error {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.Error", len)?;
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Error {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
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
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Error;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.Error")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Error, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Error {
                    message: message.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.Error", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDatabaseRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.omit_defaults {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.GetDatabaseRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.omit_defaults {
            struct_ser.serialize_field("omitDefaults", &self.omit_defaults)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDatabaseRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "omitDefaults",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            OmitDefaults,
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
                            "name" => Ok(GeneratedField::Name),
                            "omitDefaults" => Ok(GeneratedField::OmitDefaults),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDatabaseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.GetDatabaseRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetDatabaseRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name = None;
                let mut omit_defaults = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        GeneratedField::OmitDefaults => {
                            if omit_defaults.is_some() {
                                return Err(serde::de::Error::duplicate_field("omitDefaults"));
                            }
                            omit_defaults = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetDatabaseRequest {
                    name: name.unwrap_or_default(),
                    omit_defaults: omit_defaults.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.GetDatabaseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDatabaseResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rules.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.GetDatabaseResponse", len)?;
        if let Some(v) = self.rules.as_ref() {
            struct_ser.serialize_field("rules", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDatabaseResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
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
                            "rules" => Ok(GeneratedField::Rules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDatabaseResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.GetDatabaseResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetDatabaseResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetDatabaseResponse {
                    rules,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.GetDatabaseResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPartitionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        if !self.partition_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.GetPartitionRequest", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        if !self.partition_key.is_empty() {
            struct_ser.serialize_field("partitionKey", &self.partition_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPartitionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
            "partitionKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
            PartitionKey,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            "partitionKey" => Ok(GeneratedField::PartitionKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPartitionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.GetPartitionRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetPartitionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                let mut partition_key = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                        GeneratedField::PartitionKey => {
                            if partition_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionKey"));
                            }
                            partition_key = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetPartitionRequest {
                    db_name: db_name.unwrap_or_default(),
                    partition_key: partition_key.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.GetPartitionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPartitionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.partition.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.GetPartitionResponse", len)?;
        if let Some(v) = self.partition.as_ref() {
            struct_ser.serialize_field("partition", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPartitionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "partition",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Partition,
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
                            "partition" => Ok(GeneratedField::Partition),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPartitionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.GetPartitionResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetPartitionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut partition = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Partition => {
                            if partition.is_some() {
                                return Err(serde::de::Error::duplicate_field("partition"));
                            }
                            partition = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetPartitionResponse {
                    partition,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.GetPartitionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetServerIdRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.GetServerIdRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetServerIdRequest {
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
            type Value = GetServerIdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.GetServerIdRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetServerIdRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(GetServerIdRequest {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.GetServerIdRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetServerIdResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.GetServerIdResponse", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetServerIdResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetServerIdResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.GetServerIdResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetServerIdResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(GetServerIdResponse {
                    id: id.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.GetServerIdResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetServerStatusRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.GetServerStatusRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetServerStatusRequest {
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
            type Value = GetServerStatusRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.GetServerStatusRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetServerStatusRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(GetServerStatusRequest {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.GetServerStatusRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetServerStatusResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.server_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.GetServerStatusResponse", len)?;
        if let Some(v) = self.server_status.as_ref() {
            struct_ser.serialize_field("serverStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetServerStatusResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "serverStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServerStatus,
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
                            "serverStatus" => Ok(GeneratedField::ServerStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetServerStatusResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.GetServerStatusResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetServerStatusResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut server_status = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ServerStatus => {
                            if server_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverStatus"));
                            }
                            server_status = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetServerStatusResponse {
                    server_status,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.GetServerStatusResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HashRing {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.table_name {
            len += 1;
        }
        if !self.columns.is_empty() {
            len += 1;
        }
        if !self.shards.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.HashRing", len)?;
        if self.table_name {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        if !self.shards.is_empty() {
            struct_ser.serialize_field("shards", &self.shards)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HashRing {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tableName",
            "columns",
            "shards",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableName,
            Columns,
            Shards,
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
                            "shards" => Ok(GeneratedField::Shards),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HashRing;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.HashRing")
            }

            fn visit_map<V>(self, mut map: V) -> Result<HashRing, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_name = None;
                let mut columns = None;
                let mut shards = None;
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
                        GeneratedField::Shards => {
                            if shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            shards = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                    }
                }
                Ok(HashRing {
                    table_name: table_name.unwrap_or_default(),
                    columns: columns.unwrap_or_default(),
                    shards: shards.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.HashRing", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KafkaProducer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.KafkaProducer", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KafkaProducer {
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
            type Value = KafkaProducer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.KafkaProducer")
            }

            fn visit_map<V>(self, mut map: V) -> Result<KafkaProducer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(KafkaProducer {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.KafkaProducer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LifecycleRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.buffer_size_soft != 0 {
            len += 1;
        }
        if self.buffer_size_hard != 0 {
            len += 1;
        }
        if self.persist {
            len += 1;
        }
        if self.immutable {
            len += 1;
        }
        if self.worker_backoff_millis != 0 {
            len += 1;
        }
        if self.catalog_transactions_until_checkpoint != 0 {
            len += 1;
        }
        if self.catalog_transaction_prune_age.is_some() {
            len += 1;
        }
        if self.late_arrive_window_seconds != 0 {
            len += 1;
        }
        if self.persist_row_threshold != 0 {
            len += 1;
        }
        if self.persist_age_threshold_seconds != 0 {
            len += 1;
        }
        if self.mub_row_threshold != 0 {
            len += 1;
        }
        if self.parquet_cache_limit != 0 {
            len += 1;
        }
        if self.max_active_compactions_cfg.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.LifecycleRules", len)?;
        if self.buffer_size_soft != 0 {
            struct_ser.serialize_field("bufferSizeSoft", ToString::to_string(&self.buffer_size_soft).as_str())?;
        }
        if self.buffer_size_hard != 0 {
            struct_ser.serialize_field("bufferSizeHard", ToString::to_string(&self.buffer_size_hard).as_str())?;
        }
        if self.persist {
            struct_ser.serialize_field("persist", &self.persist)?;
        }
        if self.immutable {
            struct_ser.serialize_field("immutable", &self.immutable)?;
        }
        if self.worker_backoff_millis != 0 {
            struct_ser.serialize_field("workerBackoffMillis", ToString::to_string(&self.worker_backoff_millis).as_str())?;
        }
        if self.catalog_transactions_until_checkpoint != 0 {
            struct_ser.serialize_field("catalogTransactionsUntilCheckpoint", ToString::to_string(&self.catalog_transactions_until_checkpoint).as_str())?;
        }
        if let Some(v) = self.catalog_transaction_prune_age.as_ref() {
            struct_ser.serialize_field("catalogTransactionPruneAge", v)?;
        }
        if self.late_arrive_window_seconds != 0 {
            struct_ser.serialize_field("lateArriveWindowSeconds", &self.late_arrive_window_seconds)?;
        }
        if self.persist_row_threshold != 0 {
            struct_ser.serialize_field("persistRowThreshold", ToString::to_string(&self.persist_row_threshold).as_str())?;
        }
        if self.persist_age_threshold_seconds != 0 {
            struct_ser.serialize_field("persistAgeThresholdSeconds", &self.persist_age_threshold_seconds)?;
        }
        if self.mub_row_threshold != 0 {
            struct_ser.serialize_field("mubRowThreshold", ToString::to_string(&self.mub_row_threshold).as_str())?;
        }
        if self.parquet_cache_limit != 0 {
            struct_ser.serialize_field("parquetCacheLimit", ToString::to_string(&self.parquet_cache_limit).as_str())?;
        }
        if let Some(v) = self.max_active_compactions_cfg.as_ref() {
            match v {
                lifecycle_rules::MaxActiveCompactionsCfg::MaxActiveCompactions(v) => {
                    struct_ser.serialize_field("maxActiveCompactions", v)?;
                }
                lifecycle_rules::MaxActiveCompactionsCfg::MaxActiveCompactionsCpuFraction(v) => {
                    struct_ser.serialize_field("maxActiveCompactionsCpuFraction", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bufferSizeSoft",
            "bufferSizeHard",
            "persist",
            "immutable",
            "workerBackoffMillis",
            "catalogTransactionsUntilCheckpoint",
            "catalogTransactionPruneAge",
            "lateArriveWindowSeconds",
            "persistRowThreshold",
            "persistAgeThresholdSeconds",
            "mubRowThreshold",
            "parquetCacheLimit",
            "maxActiveCompactions",
            "maxActiveCompactionsCpuFraction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BufferSizeSoft,
            BufferSizeHard,
            Persist,
            Immutable,
            WorkerBackoffMillis,
            CatalogTransactionsUntilCheckpoint,
            CatalogTransactionPruneAge,
            LateArriveWindowSeconds,
            PersistRowThreshold,
            PersistAgeThresholdSeconds,
            MubRowThreshold,
            ParquetCacheLimit,
            MaxActiveCompactions,
            MaxActiveCompactionsCpuFraction,
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
                            "bufferSizeSoft" => Ok(GeneratedField::BufferSizeSoft),
                            "bufferSizeHard" => Ok(GeneratedField::BufferSizeHard),
                            "persist" => Ok(GeneratedField::Persist),
                            "immutable" => Ok(GeneratedField::Immutable),
                            "workerBackoffMillis" => Ok(GeneratedField::WorkerBackoffMillis),
                            "catalogTransactionsUntilCheckpoint" => Ok(GeneratedField::CatalogTransactionsUntilCheckpoint),
                            "catalogTransactionPruneAge" => Ok(GeneratedField::CatalogTransactionPruneAge),
                            "lateArriveWindowSeconds" => Ok(GeneratedField::LateArriveWindowSeconds),
                            "persistRowThreshold" => Ok(GeneratedField::PersistRowThreshold),
                            "persistAgeThresholdSeconds" => Ok(GeneratedField::PersistAgeThresholdSeconds),
                            "mubRowThreshold" => Ok(GeneratedField::MubRowThreshold),
                            "parquetCacheLimit" => Ok(GeneratedField::ParquetCacheLimit),
                            "maxActiveCompactions" => Ok(GeneratedField::MaxActiveCompactions),
                            "maxActiveCompactionsCpuFraction" => Ok(GeneratedField::MaxActiveCompactionsCpuFraction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LifecycleRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.LifecycleRules")
            }

            fn visit_map<V>(self, mut map: V) -> Result<LifecycleRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut buffer_size_soft = None;
                let mut buffer_size_hard = None;
                let mut persist = None;
                let mut immutable = None;
                let mut worker_backoff_millis = None;
                let mut catalog_transactions_until_checkpoint = None;
                let mut catalog_transaction_prune_age = None;
                let mut late_arrive_window_seconds = None;
                let mut persist_row_threshold = None;
                let mut persist_age_threshold_seconds = None;
                let mut mub_row_threshold = None;
                let mut parquet_cache_limit = None;
                let mut max_active_compactions_cfg = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BufferSizeSoft => {
                            if buffer_size_soft.is_some() {
                                return Err(serde::de::Error::duplicate_field("bufferSizeSoft"));
                            }
                            buffer_size_soft = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::BufferSizeHard => {
                            if buffer_size_hard.is_some() {
                                return Err(serde::de::Error::duplicate_field("bufferSizeHard"));
                            }
                            buffer_size_hard = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Persist => {
                            if persist.is_some() {
                                return Err(serde::de::Error::duplicate_field("persist"));
                            }
                            persist = Some(map.next_value()?);
                        }
                        GeneratedField::Immutable => {
                            if immutable.is_some() {
                                return Err(serde::de::Error::duplicate_field("immutable"));
                            }
                            immutable = Some(map.next_value()?);
                        }
                        GeneratedField::WorkerBackoffMillis => {
                            if worker_backoff_millis.is_some() {
                                return Err(serde::de::Error::duplicate_field("workerBackoffMillis"));
                            }
                            worker_backoff_millis = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::CatalogTransactionsUntilCheckpoint => {
                            if catalog_transactions_until_checkpoint.is_some() {
                                return Err(serde::de::Error::duplicate_field("catalogTransactionsUntilCheckpoint"));
                            }
                            catalog_transactions_until_checkpoint = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::CatalogTransactionPruneAge => {
                            if catalog_transaction_prune_age.is_some() {
                                return Err(serde::de::Error::duplicate_field("catalogTransactionPruneAge"));
                            }
                            catalog_transaction_prune_age = Some(map.next_value()?);
                        }
                        GeneratedField::LateArriveWindowSeconds => {
                            if late_arrive_window_seconds.is_some() {
                                return Err(serde::de::Error::duplicate_field("lateArriveWindowSeconds"));
                            }
                            late_arrive_window_seconds = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::PersistRowThreshold => {
                            if persist_row_threshold.is_some() {
                                return Err(serde::de::Error::duplicate_field("persistRowThreshold"));
                            }
                            persist_row_threshold = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::PersistAgeThresholdSeconds => {
                            if persist_age_threshold_seconds.is_some() {
                                return Err(serde::de::Error::duplicate_field("persistAgeThresholdSeconds"));
                            }
                            persist_age_threshold_seconds = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::MubRowThreshold => {
                            if mub_row_threshold.is_some() {
                                return Err(serde::de::Error::duplicate_field("mubRowThreshold"));
                            }
                            mub_row_threshold = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::ParquetCacheLimit => {
                            if parquet_cache_limit.is_some() {
                                return Err(serde::de::Error::duplicate_field("parquetCacheLimit"));
                            }
                            parquet_cache_limit = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::MaxActiveCompactions => {
                            if max_active_compactions_cfg.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxActiveCompactions"));
                            }
                            max_active_compactions_cfg = Some(lifecycle_rules::MaxActiveCompactionsCfg::MaxActiveCompactions(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            ));
                        }
                        GeneratedField::MaxActiveCompactionsCpuFraction => {
                            if max_active_compactions_cfg.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxActiveCompactionsCpuFraction"));
                            }
                            max_active_compactions_cfg = Some(lifecycle_rules::MaxActiveCompactionsCfg::MaxActiveCompactionsCpuFraction(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            ));
                        }
                    }
                }
                Ok(LifecycleRules {
                    buffer_size_soft: buffer_size_soft.unwrap_or_default(),
                    buffer_size_hard: buffer_size_hard.unwrap_or_default(),
                    persist: persist.unwrap_or_default(),
                    immutable: immutable.unwrap_or_default(),
                    worker_backoff_millis: worker_backoff_millis.unwrap_or_default(),
                    catalog_transactions_until_checkpoint: catalog_transactions_until_checkpoint.unwrap_or_default(),
                    catalog_transaction_prune_age,
                    late_arrive_window_seconds: late_arrive_window_seconds.unwrap_or_default(),
                    persist_row_threshold: persist_row_threshold.unwrap_or_default(),
                    persist_age_threshold_seconds: persist_age_threshold_seconds.unwrap_or_default(),
                    mub_row_threshold: mub_row_threshold.unwrap_or_default(),
                    parquet_cache_limit: parquet_cache_limit.unwrap_or_default(),
                    max_active_compactions_cfg,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.LifecycleRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListChunksRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ListChunksRequest", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListChunksRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListChunksRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ListChunksRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ListChunksRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ListChunksRequest {
                    db_name: db_name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ListChunksRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListChunksResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chunks.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ListChunksResponse", len)?;
        if !self.chunks.is_empty() {
            struct_ser.serialize_field("chunks", &self.chunks)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListChunksResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chunks",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ListChunksResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ListChunksResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ListChunksResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chunks = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Chunks => {
                            if chunks.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunks"));
                            }
                            chunks = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ListChunksResponse {
                    chunks: chunks.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ListChunksResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDatabasesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.omit_defaults {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ListDatabasesRequest", len)?;
        if self.omit_defaults {
            struct_ser.serialize_field("omitDefaults", &self.omit_defaults)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDatabasesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "omitDefaults",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OmitDefaults,
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
                            "omitDefaults" => Ok(GeneratedField::OmitDefaults),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListDatabasesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ListDatabasesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ListDatabasesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut omit_defaults = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OmitDefaults => {
                            if omit_defaults.is_some() {
                                return Err(serde::de::Error::duplicate_field("omitDefaults"));
                            }
                            omit_defaults = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ListDatabasesRequest {
                    omit_defaults: omit_defaults.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ListDatabasesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDatabasesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ListDatabasesResponse", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDatabasesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
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
                            "rules" => Ok(GeneratedField::Rules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListDatabasesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ListDatabasesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ListDatabasesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ListDatabasesResponse {
                    rules: rules.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ListDatabasesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDeletedDatabasesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ListDeletedDatabasesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDeletedDatabasesRequest {
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
            type Value = ListDeletedDatabasesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ListDeletedDatabasesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ListDeletedDatabasesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(ListDeletedDatabasesRequest {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ListDeletedDatabasesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDeletedDatabasesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.deleted_databases.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ListDeletedDatabasesResponse", len)?;
        if !self.deleted_databases.is_empty() {
            struct_ser.serialize_field("deletedDatabases", &self.deleted_databases)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDeletedDatabasesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deletedDatabases",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeletedDatabases,
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
                            "deletedDatabases" => Ok(GeneratedField::DeletedDatabases),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListDeletedDatabasesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ListDeletedDatabasesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ListDeletedDatabasesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deleted_databases = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DeletedDatabases => {
                            if deleted_databases.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletedDatabases"));
                            }
                            deleted_databases = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ListDeletedDatabasesResponse {
                    deleted_databases: deleted_databases.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ListDeletedDatabasesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDetailedDatabasesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ListDetailedDatabasesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDetailedDatabasesRequest {
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
            type Value = ListDetailedDatabasesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ListDetailedDatabasesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ListDetailedDatabasesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(ListDetailedDatabasesRequest {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ListDetailedDatabasesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDetailedDatabasesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.databases.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ListDetailedDatabasesResponse", len)?;
        if !self.databases.is_empty() {
            struct_ser.serialize_field("databases", &self.databases)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDetailedDatabasesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "databases",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Databases,
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
                            "databases" => Ok(GeneratedField::Databases),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListDetailedDatabasesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ListDetailedDatabasesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ListDetailedDatabasesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut databases = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Databases => {
                            if databases.is_some() {
                                return Err(serde::de::Error::duplicate_field("databases"));
                            }
                            databases = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ListDetailedDatabasesResponse {
                    databases: databases.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ListDetailedDatabasesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPartitionChunksRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        if !self.partition_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ListPartitionChunksRequest", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        if !self.partition_key.is_empty() {
            struct_ser.serialize_field("partitionKey", &self.partition_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPartitionChunksRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
            "partitionKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
            PartitionKey,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            "partitionKey" => Ok(GeneratedField::PartitionKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListPartitionChunksRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ListPartitionChunksRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ListPartitionChunksRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                let mut partition_key = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                        GeneratedField::PartitionKey => {
                            if partition_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionKey"));
                            }
                            partition_key = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ListPartitionChunksRequest {
                    db_name: db_name.unwrap_or_default(),
                    partition_key: partition_key.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ListPartitionChunksRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPartitionChunksResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chunks.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ListPartitionChunksResponse", len)?;
        if !self.chunks.is_empty() {
            struct_ser.serialize_field("chunks", &self.chunks)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPartitionChunksResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chunks",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ListPartitionChunksResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ListPartitionChunksResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ListPartitionChunksResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chunks = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Chunks => {
                            if chunks.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunks"));
                            }
                            chunks = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ListPartitionChunksResponse {
                    chunks: chunks.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ListPartitionChunksResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPartitionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ListPartitionsRequest", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPartitionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListPartitionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ListPartitionsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ListPartitionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ListPartitionsRequest {
                    db_name: db_name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ListPartitionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPartitionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.partitions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ListPartitionsResponse", len)?;
        if !self.partitions.is_empty() {
            struct_ser.serialize_field("partitions", &self.partitions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPartitionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "partitions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Partitions,
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
                            "partitions" => Ok(GeneratedField::Partitions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListPartitionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ListPartitionsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ListPartitionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut partitions = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Partitions => {
                            if partitions.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitions"));
                            }
                            partitions = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ListPartitionsResponse {
                    partitions: partitions.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ListPartitionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRemotesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ListRemotesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRemotesRequest {
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
            type Value = ListRemotesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ListRemotesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ListRemotesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(ListRemotesRequest {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ListRemotesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRemotesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.remotes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ListRemotesResponse", len)?;
        if !self.remotes.is_empty() {
            struct_ser.serialize_field("remotes", &self.remotes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRemotesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remotes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Remotes,
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
                            "remotes" => Ok(GeneratedField::Remotes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListRemotesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ListRemotesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ListRemotesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut remotes = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Remotes => {
                            if remotes.is_some() {
                                return Err(serde::de::Error::duplicate_field("remotes"));
                            }
                            remotes = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ListRemotesResponse {
                    remotes: remotes.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ListRemotesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Matcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.table_name_regex.is_empty() {
            len += 1;
        }
        if !self.predicate.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.Matcher", len)?;
        if !self.table_name_regex.is_empty() {
            struct_ser.serialize_field("tableNameRegex", &self.table_name_regex)?;
        }
        if !self.predicate.is_empty() {
            struct_ser.serialize_field("predicate", &self.predicate)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Matcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tableNameRegex",
            "predicate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableNameRegex,
            Predicate,
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
                            "tableNameRegex" => Ok(GeneratedField::TableNameRegex),
                            "predicate" => Ok(GeneratedField::Predicate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Matcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.Matcher")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Matcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_name_regex = None;
                let mut predicate = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TableNameRegex => {
                            if table_name_regex.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableNameRegex"));
                            }
                            table_name_regex = Some(map.next_value()?);
                        }
                        GeneratedField::Predicate => {
                            if predicate.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Matcher {
                    table_name_regex: table_name_regex.unwrap_or_default(),
                    predicate: predicate.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.Matcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MatcherToShard {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.matcher.is_some() {
            len += 1;
        }
        if self.shard != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.MatcherToShard", len)?;
        if let Some(v) = self.matcher.as_ref() {
            struct_ser.serialize_field("matcher", v)?;
        }
        if self.shard != 0 {
            struct_ser.serialize_field("shard", &self.shard)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MatcherToShard {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "matcher",
            "shard",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Matcher,
            Shard,
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
                            "matcher" => Ok(GeneratedField::Matcher),
                            "shard" => Ok(GeneratedField::Shard),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MatcherToShard;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.MatcherToShard")
            }

            fn visit_map<V>(self, mut map: V) -> Result<MatcherToShard, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut matcher = None;
                let mut shard = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Matcher => {
                            if matcher.is_some() {
                                return Err(serde::de::Error::duplicate_field("matcher"));
                            }
                            matcher = Some(map.next_value()?);
                        }
                        GeneratedField::Shard => {
                            if shard.is_some() {
                                return Err(serde::de::Error::duplicate_field("shard"));
                            }
                            shard = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(MatcherToShard {
                    matcher,
                    shard: shard.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.MatcherToShard", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NewPartitionChunkRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        if !self.partition_key.is_empty() {
            len += 1;
        }
        if !self.table_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.NewPartitionChunkRequest", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        if !self.partition_key.is_empty() {
            struct_ser.serialize_field("partitionKey", &self.partition_key)?;
        }
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NewPartitionChunkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
            "partitionKey",
            "tableName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
            PartitionKey,
            TableName,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            "partitionKey" => Ok(GeneratedField::PartitionKey),
                            "tableName" => Ok(GeneratedField::TableName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NewPartitionChunkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.NewPartitionChunkRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<NewPartitionChunkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                let mut partition_key = None;
                let mut table_name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                        GeneratedField::PartitionKey => {
                            if partition_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionKey"));
                            }
                            partition_key = Some(map.next_value()?);
                        }
                        GeneratedField::TableName => {
                            if table_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(NewPartitionChunkRequest {
                    db_name: db_name.unwrap_or_default(),
                    partition_key: partition_key.unwrap_or_default(),
                    table_name: table_name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.NewPartitionChunkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NewPartitionChunkResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.NewPartitionChunkResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NewPartitionChunkResponse {
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
            type Value = NewPartitionChunkResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.NewPartitionChunkResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<NewPartitionChunkResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(NewPartitionChunkResponse {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.NewPartitionChunkResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NodeGroup {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.nodes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.NodeGroup", len)?;
        if !self.nodes.is_empty() {
            struct_ser.serialize_field("nodes", &self.nodes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NodeGroup {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nodes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nodes,
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
                            "nodes" => Ok(GeneratedField::Nodes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NodeGroup;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.NodeGroup")
            }

            fn visit_map<V>(self, mut map: V) -> Result<NodeGroup, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut nodes = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Nodes => {
                            if nodes.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            nodes = Some(map.next_value()?);
                        }
                    }
                }
                Ok(NodeGroup {
                    nodes: nodes.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.NodeGroup", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for node_group::Node {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.NodeGroup.Node", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for node_group::Node {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = node_group::Node;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.NodeGroup.Node")
            }

            fn visit_map<V>(self, mut map: V) -> Result<node_group::Node, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(node_group::Node {
                    id: id.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.NodeGroup.Node", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OperationMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cpu_nanos != 0 {
            len += 1;
        }
        if self.wall_nanos != 0 {
            len += 1;
        }
        if self.total_count != 0 {
            len += 1;
        }
        if self.pending_count != 0 {
            len += 1;
        }
        if self.success_count != 0 {
            len += 1;
        }
        if self.error_count != 0 {
            len += 1;
        }
        if self.cancelled_count != 0 {
            len += 1;
        }
        if self.dropped_count != 0 {
            len += 1;
        }
        if self.job.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.OperationMetadata", len)?;
        if self.cpu_nanos != 0 {
            struct_ser.serialize_field("cpuNanos", ToString::to_string(&self.cpu_nanos).as_str())?;
        }
        if self.wall_nanos != 0 {
            struct_ser.serialize_field("wallNanos", ToString::to_string(&self.wall_nanos).as_str())?;
        }
        if self.total_count != 0 {
            struct_ser.serialize_field("totalCount", ToString::to_string(&self.total_count).as_str())?;
        }
        if self.pending_count != 0 {
            struct_ser.serialize_field("pendingCount", ToString::to_string(&self.pending_count).as_str())?;
        }
        if self.success_count != 0 {
            struct_ser.serialize_field("successCount", ToString::to_string(&self.success_count).as_str())?;
        }
        if self.error_count != 0 {
            struct_ser.serialize_field("errorCount", ToString::to_string(&self.error_count).as_str())?;
        }
        if self.cancelled_count != 0 {
            struct_ser.serialize_field("cancelledCount", ToString::to_string(&self.cancelled_count).as_str())?;
        }
        if self.dropped_count != 0 {
            struct_ser.serialize_field("droppedCount", ToString::to_string(&self.dropped_count).as_str())?;
        }
        if let Some(v) = self.job.as_ref() {
            match v {
                operation_metadata::Job::Dummy(v) => {
                    struct_ser.serialize_field("dummy", v)?;
                }
                operation_metadata::Job::WriteChunk(v) => {
                    struct_ser.serialize_field("writeChunk", v)?;
                }
                operation_metadata::Job::WipePreservedCatalog(v) => {
                    struct_ser.serialize_field("wipePreservedCatalog", v)?;
                }
                operation_metadata::Job::CompactChunks(v) => {
                    struct_ser.serialize_field("compactChunks", v)?;
                }
                operation_metadata::Job::PersistChunks(v) => {
                    struct_ser.serialize_field("persistChunks", v)?;
                }
                operation_metadata::Job::DropChunk(v) => {
                    struct_ser.serialize_field("dropChunk", v)?;
                }
                operation_metadata::Job::DropPartition(v) => {
                    struct_ser.serialize_field("dropPartition", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OperationMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cpuNanos",
            "wallNanos",
            "totalCount",
            "pendingCount",
            "successCount",
            "errorCount",
            "cancelledCount",
            "droppedCount",
            "dummy",
            "writeChunk",
            "wipePreservedCatalog",
            "compactChunks",
            "persistChunks",
            "dropChunk",
            "dropPartition",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CpuNanos,
            WallNanos,
            TotalCount,
            PendingCount,
            SuccessCount,
            ErrorCount,
            CancelledCount,
            DroppedCount,
            Dummy,
            WriteChunk,
            WipePreservedCatalog,
            CompactChunks,
            PersistChunks,
            DropChunk,
            DropPartition,
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
                            "cpuNanos" => Ok(GeneratedField::CpuNanos),
                            "wallNanos" => Ok(GeneratedField::WallNanos),
                            "totalCount" => Ok(GeneratedField::TotalCount),
                            "pendingCount" => Ok(GeneratedField::PendingCount),
                            "successCount" => Ok(GeneratedField::SuccessCount),
                            "errorCount" => Ok(GeneratedField::ErrorCount),
                            "cancelledCount" => Ok(GeneratedField::CancelledCount),
                            "droppedCount" => Ok(GeneratedField::DroppedCount),
                            "dummy" => Ok(GeneratedField::Dummy),
                            "writeChunk" => Ok(GeneratedField::WriteChunk),
                            "wipePreservedCatalog" => Ok(GeneratedField::WipePreservedCatalog),
                            "compactChunks" => Ok(GeneratedField::CompactChunks),
                            "persistChunks" => Ok(GeneratedField::PersistChunks),
                            "dropChunk" => Ok(GeneratedField::DropChunk),
                            "dropPartition" => Ok(GeneratedField::DropPartition),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OperationMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.OperationMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> Result<OperationMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cpu_nanos = None;
                let mut wall_nanos = None;
                let mut total_count = None;
                let mut pending_count = None;
                let mut success_count = None;
                let mut error_count = None;
                let mut cancelled_count = None;
                let mut dropped_count = None;
                let mut job = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CpuNanos => {
                            if cpu_nanos.is_some() {
                                return Err(serde::de::Error::duplicate_field("cpuNanos"));
                            }
                            cpu_nanos = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::WallNanos => {
                            if wall_nanos.is_some() {
                                return Err(serde::de::Error::duplicate_field("wallNanos"));
                            }
                            wall_nanos = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::TotalCount => {
                            if total_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalCount"));
                            }
                            total_count = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::PendingCount => {
                            if pending_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("pendingCount"));
                            }
                            pending_count = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::SuccessCount => {
                            if success_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("successCount"));
                            }
                            success_count = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::ErrorCount => {
                            if error_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorCount"));
                            }
                            error_count = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::CancelledCount => {
                            if cancelled_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelledCount"));
                            }
                            cancelled_count = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::DroppedCount => {
                            if dropped_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("droppedCount"));
                            }
                            dropped_count = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Dummy => {
                            if job.is_some() {
                                return Err(serde::de::Error::duplicate_field("dummy"));
                            }
                            job = Some(operation_metadata::Job::Dummy(map.next_value()?));
                        }
                        GeneratedField::WriteChunk => {
                            if job.is_some() {
                                return Err(serde::de::Error::duplicate_field("writeChunk"));
                            }
                            job = Some(operation_metadata::Job::WriteChunk(map.next_value()?));
                        }
                        GeneratedField::WipePreservedCatalog => {
                            if job.is_some() {
                                return Err(serde::de::Error::duplicate_field("wipePreservedCatalog"));
                            }
                            job = Some(operation_metadata::Job::WipePreservedCatalog(map.next_value()?));
                        }
                        GeneratedField::CompactChunks => {
                            if job.is_some() {
                                return Err(serde::de::Error::duplicate_field("compactChunks"));
                            }
                            job = Some(operation_metadata::Job::CompactChunks(map.next_value()?));
                        }
                        GeneratedField::PersistChunks => {
                            if job.is_some() {
                                return Err(serde::de::Error::duplicate_field("persistChunks"));
                            }
                            job = Some(operation_metadata::Job::PersistChunks(map.next_value()?));
                        }
                        GeneratedField::DropChunk => {
                            if job.is_some() {
                                return Err(serde::de::Error::duplicate_field("dropChunk"));
                            }
                            job = Some(operation_metadata::Job::DropChunk(map.next_value()?));
                        }
                        GeneratedField::DropPartition => {
                            if job.is_some() {
                                return Err(serde::de::Error::duplicate_field("dropPartition"));
                            }
                            job = Some(operation_metadata::Job::DropPartition(map.next_value()?));
                        }
                    }
                }
                Ok(OperationMetadata {
                    cpu_nanos: cpu_nanos.unwrap_or_default(),
                    wall_nanos: wall_nanos.unwrap_or_default(),
                    total_count: total_count.unwrap_or_default(),
                    pending_count: pending_count.unwrap_or_default(),
                    success_count: success_count.unwrap_or_default(),
                    error_count: error_count.unwrap_or_default(),
                    cancelled_count: cancelled_count.unwrap_or_default(),
                    dropped_count: dropped_count.unwrap_or_default(),
                    job,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.OperationMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Partition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.Partition", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Partition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
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
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Partition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.Partition")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Partition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Partition {
                    key: key.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.Partition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PartitionTemplate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.PartitionTemplate", len)?;
        if !self.parts.is_empty() {
            struct_ser.serialize_field("parts", &self.parts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PartitionTemplate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Parts,
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
                            "parts" => Ok(GeneratedField::Parts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PartitionTemplate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.PartitionTemplate")
            }

            fn visit_map<V>(self, mut map: V) -> Result<PartitionTemplate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parts = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Parts => {
                            if parts.is_some() {
                                return Err(serde::de::Error::duplicate_field("parts"));
                            }
                            parts = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PartitionTemplate {
                    parts: parts.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.PartitionTemplate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for partition_template::Part {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.part.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.PartitionTemplate.Part", len)?;
        if let Some(v) = self.part.as_ref() {
            match v {
                partition_template::part::Part::Table(v) => {
                    struct_ser.serialize_field("table", v)?;
                }
                partition_template::part::Part::Column(v) => {
                    struct_ser.serialize_field("column", v)?;
                }
                partition_template::part::Part::Time(v) => {
                    struct_ser.serialize_field("time", v)?;
                }
                partition_template::part::Part::Regex(v) => {
                    struct_ser.serialize_field("regex", v)?;
                }
                partition_template::part::Part::StrfTime(v) => {
                    struct_ser.serialize_field("strfTime", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for partition_template::Part {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table",
            "column",
            "time",
            "regex",
            "strfTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Table,
            Column,
            Time,
            Regex,
            StrfTime,
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
                            "table" => Ok(GeneratedField::Table),
                            "column" => Ok(GeneratedField::Column),
                            "time" => Ok(GeneratedField::Time),
                            "regex" => Ok(GeneratedField::Regex),
                            "strfTime" => Ok(GeneratedField::StrfTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = partition_template::Part;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.PartitionTemplate.Part")
            }

            fn visit_map<V>(self, mut map: V) -> Result<partition_template::Part, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut part = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Table => {
                            if part.is_some() {
                                return Err(serde::de::Error::duplicate_field("table"));
                            }
                            part = Some(partition_template::part::Part::Table(map.next_value()?));
                        }
                        GeneratedField::Column => {
                            if part.is_some() {
                                return Err(serde::de::Error::duplicate_field("column"));
                            }
                            part = Some(partition_template::part::Part::Column(map.next_value()?));
                        }
                        GeneratedField::Time => {
                            if part.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            part = Some(partition_template::part::Part::Time(map.next_value()?));
                        }
                        GeneratedField::Regex => {
                            if part.is_some() {
                                return Err(serde::de::Error::duplicate_field("regex"));
                            }
                            part = Some(partition_template::part::Part::Regex(map.next_value()?));
                        }
                        GeneratedField::StrfTime => {
                            if part.is_some() {
                                return Err(serde::de::Error::duplicate_field("strfTime"));
                            }
                            part = Some(partition_template::part::Part::StrfTime(map.next_value()?));
                        }
                    }
                }
                Ok(partition_template::Part {
                    part,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.PartitionTemplate.Part", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for partition_template::part::ColumnFormat {
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
        if !self.format.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.PartitionTemplate.Part.ColumnFormat", len)?;
        if !self.column.is_empty() {
            struct_ser.serialize_field("column", &self.column)?;
        }
        if !self.format.is_empty() {
            struct_ser.serialize_field("format", &self.format)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for partition_template::part::ColumnFormat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "column",
            "format",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Column,
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
                            "column" => Ok(GeneratedField::Column),
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
            type Value = partition_template::part::ColumnFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.PartitionTemplate.Part.ColumnFormat")
            }

            fn visit_map<V>(self, mut map: V) -> Result<partition_template::part::ColumnFormat, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut column = None;
                let mut format = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Column => {
                            if column.is_some() {
                                return Err(serde::de::Error::duplicate_field("column"));
                            }
                            column = Some(map.next_value()?);
                        }
                        GeneratedField::Format => {
                            if format.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            format = Some(map.next_value()?);
                        }
                    }
                }
                Ok(partition_template::part::ColumnFormat {
                    column: column.unwrap_or_default(),
                    format: format.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.PartitionTemplate.Part.ColumnFormat", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PersistChunks {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        if !self.partition_key.is_empty() {
            len += 1;
        }
        if !self.table_name.is_empty() {
            len += 1;
        }
        if !self.chunks.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.PersistChunks", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        if !self.partition_key.is_empty() {
            struct_ser.serialize_field("partitionKey", &self.partition_key)?;
        }
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if !self.chunks.is_empty() {
            struct_ser.serialize_field("chunks", &self.chunks)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PersistChunks {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
            "partitionKey",
            "tableName",
            "chunks",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
            PartitionKey,
            TableName,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            "partitionKey" => Ok(GeneratedField::PartitionKey),
                            "tableName" => Ok(GeneratedField::TableName),
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
            type Value = PersistChunks;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.PersistChunks")
            }

            fn visit_map<V>(self, mut map: V) -> Result<PersistChunks, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                let mut partition_key = None;
                let mut table_name = None;
                let mut chunks = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                        GeneratedField::PartitionKey => {
                            if partition_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionKey"));
                            }
                            partition_key = Some(map.next_value()?);
                        }
                        GeneratedField::TableName => {
                            if table_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name = Some(map.next_value()?);
                        }
                        GeneratedField::Chunks => {
                            if chunks.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunks"));
                            }
                            chunks = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                    }
                }
                Ok(PersistChunks {
                    db_name: db_name.unwrap_or_default(),
                    partition_key: partition_key.unwrap_or_default(),
                    table_name: table_name.unwrap_or_default(),
                    chunks: chunks.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.PersistChunks", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PersistPartitionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        if !self.partition_key.is_empty() {
            len += 1;
        }
        if !self.table_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.PersistPartitionRequest", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        if !self.partition_key.is_empty() {
            struct_ser.serialize_field("partitionKey", &self.partition_key)?;
        }
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PersistPartitionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
            "partitionKey",
            "tableName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
            PartitionKey,
            TableName,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            "partitionKey" => Ok(GeneratedField::PartitionKey),
                            "tableName" => Ok(GeneratedField::TableName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PersistPartitionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.PersistPartitionRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<PersistPartitionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                let mut partition_key = None;
                let mut table_name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                        GeneratedField::PartitionKey => {
                            if partition_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionKey"));
                            }
                            partition_key = Some(map.next_value()?);
                        }
                        GeneratedField::TableName => {
                            if table_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PersistPartitionRequest {
                    db_name: db_name.unwrap_or_default(),
                    partition_key: partition_key.unwrap_or_default(),
                    table_name: table_name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.PersistPartitionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PersistPartitionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.PersistPartitionResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PersistPartitionResponse {
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
            type Value = PersistPartitionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.PersistPartitionResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<PersistPartitionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(PersistPartitionResponse {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.PersistPartitionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Remote {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.connection_string.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.Remote", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.connection_string.is_empty() {
            struct_ser.serialize_field("connectionString", &self.connection_string)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Remote {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "connectionString",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ConnectionString,
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
                            "id" => Ok(GeneratedField::Id),
                            "connectionString" => Ok(GeneratedField::ConnectionString),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Remote;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.Remote")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Remote, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut connection_string = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::ConnectionString => {
                            if connection_string.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionString"));
                            }
                            connection_string = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Remote {
                    id: id.unwrap_or_default(),
                    connection_string: connection_string.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.Remote", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RestoreDatabaseRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.generation_id != 0 {
            len += 1;
        }
        if !self.db_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.RestoreDatabaseRequest", len)?;
        if self.generation_id != 0 {
            struct_ser.serialize_field("generationId", ToString::to_string(&self.generation_id).as_str())?;
        }
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RestoreDatabaseRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "generationId",
            "dbName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GenerationId,
            DbName,
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
                            "generationId" => Ok(GeneratedField::GenerationId),
                            "dbName" => Ok(GeneratedField::DbName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RestoreDatabaseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.RestoreDatabaseRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<RestoreDatabaseRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut generation_id = None;
                let mut db_name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GenerationId => {
                            if generation_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("generationId"));
                            }
                            generation_id = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RestoreDatabaseRequest {
                    generation_id: generation_id.unwrap_or_default(),
                    db_name: db_name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.RestoreDatabaseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RestoreDatabaseResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.RestoreDatabaseResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RestoreDatabaseResponse {
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
            type Value = RestoreDatabaseResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.RestoreDatabaseResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<RestoreDatabaseResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(RestoreDatabaseResponse {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.RestoreDatabaseResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoutingConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sink.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.RoutingConfig", len)?;
        if let Some(v) = self.sink.as_ref() {
            struct_ser.serialize_field("sink", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoutingConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sink",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sink,
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
                            "sink" => Ok(GeneratedField::Sink),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoutingConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.RoutingConfig")
            }

            fn visit_map<V>(self, mut map: V) -> Result<RoutingConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sink = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Sink => {
                            if sink.is_some() {
                                return Err(serde::de::Error::duplicate_field("sink"));
                            }
                            sink = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RoutingConfig {
                    sink,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.RoutingConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServerStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.initialized {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        if !self.database_statuses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ServerStatus", len)?;
        if self.initialized {
            struct_ser.serialize_field("initialized", &self.initialized)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        if !self.database_statuses.is_empty() {
            struct_ser.serialize_field("databaseStatuses", &self.database_statuses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServerStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "initialized",
            "error",
            "databaseStatuses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Initialized,
            Error,
            DatabaseStatuses,
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
                            "initialized" => Ok(GeneratedField::Initialized),
                            "error" => Ok(GeneratedField::Error),
                            "databaseStatuses" => Ok(GeneratedField::DatabaseStatuses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServerStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ServerStatus")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ServerStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut initialized = None;
                let mut error = None;
                let mut database_statuses = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Initialized => {
                            if initialized.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialized"));
                            }
                            initialized = Some(map.next_value()?);
                        }
                        GeneratedField::Error => {
                            if error.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error = Some(map.next_value()?);
                        }
                        GeneratedField::DatabaseStatuses => {
                            if database_statuses.is_some() {
                                return Err(serde::de::Error::duplicate_field("databaseStatuses"));
                            }
                            database_statuses = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ServerStatus {
                    initialized: initialized.unwrap_or_default(),
                    error,
                    database_statuses: database_statuses.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ServerStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetServingReadinessRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ready {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.SetServingReadinessRequest", len)?;
        if self.ready {
            struct_ser.serialize_field("ready", &self.ready)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetServingReadinessRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ready",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ready,
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
                            "ready" => Ok(GeneratedField::Ready),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetServingReadinessRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.SetServingReadinessRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<SetServingReadinessRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ready = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ready => {
                            if ready.is_some() {
                                return Err(serde::de::Error::duplicate_field("ready"));
                            }
                            ready = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SetServingReadinessRequest {
                    ready: ready.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.SetServingReadinessRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetServingReadinessResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.SetServingReadinessResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetServingReadinessResponse {
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
            type Value = SetServingReadinessResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.SetServingReadinessResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<SetServingReadinessResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(SetServingReadinessResponse {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.SetServingReadinessResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShardConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.specific_targets.is_empty() {
            len += 1;
        }
        if self.hash_ring.is_some() {
            len += 1;
        }
        if self.ignore_errors {
            len += 1;
        }
        if !self.shards.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.ShardConfig", len)?;
        if !self.specific_targets.is_empty() {
            struct_ser.serialize_field("specificTargets", &self.specific_targets)?;
        }
        if let Some(v) = self.hash_ring.as_ref() {
            struct_ser.serialize_field("hashRing", v)?;
        }
        if self.ignore_errors {
            struct_ser.serialize_field("ignoreErrors", &self.ignore_errors)?;
        }
        if !self.shards.is_empty() {
            struct_ser.serialize_field("shards", &self.shards)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShardConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "specificTargets",
            "hashRing",
            "ignoreErrors",
            "shards",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SpecificTargets,
            HashRing,
            IgnoreErrors,
            Shards,
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
                            "specificTargets" => Ok(GeneratedField::SpecificTargets),
                            "hashRing" => Ok(GeneratedField::HashRing),
                            "ignoreErrors" => Ok(GeneratedField::IgnoreErrors),
                            "shards" => Ok(GeneratedField::Shards),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ShardConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.ShardConfig")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ShardConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut specific_targets = None;
                let mut hash_ring = None;
                let mut ignore_errors = None;
                let mut shards = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SpecificTargets => {
                            if specific_targets.is_some() {
                                return Err(serde::de::Error::duplicate_field("specificTargets"));
                            }
                            specific_targets = Some(map.next_value()?);
                        }
                        GeneratedField::HashRing => {
                            if hash_ring.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashRing"));
                            }
                            hash_ring = Some(map.next_value()?);
                        }
                        GeneratedField::IgnoreErrors => {
                            if ignore_errors.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreErrors"));
                            }
                            ignore_errors = Some(map.next_value()?);
                        }
                        GeneratedField::Shards => {
                            if shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("shards"));
                            }
                            shards = Some(
                                map.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<u32>, _>>()?
                                    .into_iter().map(|(k,v)| (k.0, v)).collect()
                            );
                        }
                    }
                }
                Ok(ShardConfig {
                    specific_targets: specific_targets.unwrap_or_default(),
                    hash_ring,
                    ignore_errors: ignore_errors.unwrap_or_default(),
                    shards: shards.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.ShardConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Sink {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sink.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.Sink", len)?;
        if let Some(v) = self.sink.as_ref() {
            match v {
                sink::Sink::Iox(v) => {
                    struct_ser.serialize_field("iox", v)?;
                }
                sink::Sink::Kafka(v) => {
                    struct_ser.serialize_field("kafka", v)?;
                }
                sink::Sink::DevNull(v) => {
                    struct_ser.serialize_field("devNull", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Sink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "iox",
            "kafka",
            "devNull",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Iox,
            Kafka,
            DevNull,
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
                            "iox" => Ok(GeneratedField::Iox),
                            "kafka" => Ok(GeneratedField::Kafka),
                            "devNull" => Ok(GeneratedField::DevNull),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Sink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.Sink")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Sink, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sink = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Iox => {
                            if sink.is_some() {
                                return Err(serde::de::Error::duplicate_field("iox"));
                            }
                            sink = Some(sink::Sink::Iox(map.next_value()?));
                        }
                        GeneratedField::Kafka => {
                            if sink.is_some() {
                                return Err(serde::de::Error::duplicate_field("kafka"));
                            }
                            sink = Some(sink::Sink::Kafka(map.next_value()?));
                        }
                        GeneratedField::DevNull => {
                            if sink.is_some() {
                                return Err(serde::de::Error::duplicate_field("devNull"));
                            }
                            sink = Some(sink::Sink::DevNull(map.next_value()?));
                        }
                    }
                }
                Ok(Sink {
                    sink,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.Sink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SkipReplayRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.SkipReplayRequest", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SkipReplayRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SkipReplayRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.SkipReplayRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<SkipReplayRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SkipReplayRequest {
                    db_name: db_name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.SkipReplayRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SkipReplayResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.SkipReplayResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SkipReplayResponse {
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
            type Value = SkipReplayResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.SkipReplayResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<SkipReplayResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(SkipReplayResponse {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.SkipReplayResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnloadPartitionChunkRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        if !self.partition_key.is_empty() {
            len += 1;
        }
        if !self.table_name.is_empty() {
            len += 1;
        }
        if self.chunk_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.UnloadPartitionChunkRequest", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        if !self.partition_key.is_empty() {
            struct_ser.serialize_field("partitionKey", &self.partition_key)?;
        }
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if self.chunk_id != 0 {
            struct_ser.serialize_field("chunkId", &self.chunk_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnloadPartitionChunkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
            "partitionKey",
            "tableName",
            "chunkId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
            PartitionKey,
            TableName,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            "partitionKey" => Ok(GeneratedField::PartitionKey),
                            "tableName" => Ok(GeneratedField::TableName),
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
            type Value = UnloadPartitionChunkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.UnloadPartitionChunkRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<UnloadPartitionChunkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                let mut partition_key = None;
                let mut table_name = None;
                let mut chunk_id = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                        GeneratedField::PartitionKey => {
                            if partition_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionKey"));
                            }
                            partition_key = Some(map.next_value()?);
                        }
                        GeneratedField::TableName => {
                            if table_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name = Some(map.next_value()?);
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
                Ok(UnloadPartitionChunkRequest {
                    db_name: db_name.unwrap_or_default(),
                    partition_key: partition_key.unwrap_or_default(),
                    table_name: table_name.unwrap_or_default(),
                    chunk_id: chunk_id.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.UnloadPartitionChunkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnloadPartitionChunkResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.UnloadPartitionChunkResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnloadPartitionChunkResponse {
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
            type Value = UnloadPartitionChunkResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.UnloadPartitionChunkResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<UnloadPartitionChunkResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(UnloadPartitionChunkResponse {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.UnloadPartitionChunkResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateDatabaseRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rules.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.UpdateDatabaseRequest", len)?;
        if let Some(v) = self.rules.as_ref() {
            struct_ser.serialize_field("rules", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateDatabaseRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
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
                            "rules" => Ok(GeneratedField::Rules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateDatabaseRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.UpdateDatabaseRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<UpdateDatabaseRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdateDatabaseRequest {
                    rules,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.UpdateDatabaseRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateDatabaseResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rules.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.UpdateDatabaseResponse", len)?;
        if let Some(v) = self.rules.as_ref() {
            struct_ser.serialize_field("rules", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateDatabaseResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
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
                            "rules" => Ok(GeneratedField::Rules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateDatabaseResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.UpdateDatabaseResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<UpdateDatabaseResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdateDatabaseResponse {
                    rules,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.UpdateDatabaseResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRemoteRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.remote.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.UpdateRemoteRequest", len)?;
        if let Some(v) = self.remote.as_ref() {
            struct_ser.serialize_field("remote", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRemoteRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remote",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Remote,
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
                            "remote" => Ok(GeneratedField::Remote),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateRemoteRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.UpdateRemoteRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<UpdateRemoteRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut remote = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Remote => {
                            if remote.is_some() {
                                return Err(serde::de::Error::duplicate_field("remote"));
                            }
                            remote = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdateRemoteRequest {
                    remote,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.UpdateRemoteRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRemoteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.UpdateRemoteResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRemoteResponse {
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
            type Value = UpdateRemoteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.UpdateRemoteResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<UpdateRemoteResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(UpdateRemoteResponse {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.UpdateRemoteResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateServerIdRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.UpdateServerIdRequest", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateServerIdRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateServerIdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.UpdateServerIdRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<UpdateServerIdRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(UpdateServerIdRequest {
                    id: id.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.UpdateServerIdRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateServerIdResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.UpdateServerIdResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateServerIdResponse {
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
            type Value = UpdateServerIdResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.UpdateServerIdResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<UpdateServerIdResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(UpdateServerIdResponse {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.UpdateServerIdResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WipePreservedCatalog {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.WipePreservedCatalog", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WipePreservedCatalog {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WipePreservedCatalog;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.WipePreservedCatalog")
            }

            fn visit_map<V>(self, mut map: V) -> Result<WipePreservedCatalog, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(WipePreservedCatalog {
                    db_name: db_name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.WipePreservedCatalog", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WipePreservedCatalogRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.WipePreservedCatalogRequest", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WipePreservedCatalogRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WipePreservedCatalogRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.WipePreservedCatalogRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<WipePreservedCatalogRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(WipePreservedCatalogRequest {
                    db_name: db_name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.WipePreservedCatalogRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WipePreservedCatalogResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.operation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.WipePreservedCatalogResponse", len)?;
        if let Some(v) = self.operation.as_ref() {
            struct_ser.serialize_field("operation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WipePreservedCatalogResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operation,
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
                            "operation" => Ok(GeneratedField::Operation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WipePreservedCatalogResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.WipePreservedCatalogResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<WipePreservedCatalogResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operation = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Operation => {
                            if operation.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation = Some(map.next_value()?);
                        }
                    }
                }
                Ok(WipePreservedCatalogResponse {
                    operation,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.WipePreservedCatalogResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteBufferConnection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.direction != 0 {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.connection.is_empty() {
            len += 1;
        }
        if !self.connection_config.is_empty() {
            len += 1;
        }
        if self.creation_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.WriteBufferConnection", len)?;
        if self.direction != 0 {
            let v = write_buffer_connection::Direction::from_i32(self.direction)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.direction)))?;
            struct_ser.serialize_field("direction", &v)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.connection.is_empty() {
            struct_ser.serialize_field("connection", &self.connection)?;
        }
        if !self.connection_config.is_empty() {
            struct_ser.serialize_field("connectionConfig", &self.connection_config)?;
        }
        if let Some(v) = self.creation_config.as_ref() {
            struct_ser.serialize_field("creationConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteBufferConnection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "direction",
            "type",
            "connection",
            "connectionConfig",
            "creationConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Direction,
            Type,
            Connection,
            ConnectionConfig,
            CreationConfig,
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
                            "direction" => Ok(GeneratedField::Direction),
                            "type" => Ok(GeneratedField::Type),
                            "connection" => Ok(GeneratedField::Connection),
                            "connectionConfig" => Ok(GeneratedField::ConnectionConfig),
                            "creationConfig" => Ok(GeneratedField::CreationConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteBufferConnection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.WriteBufferConnection")
            }

            fn visit_map<V>(self, mut map: V) -> Result<WriteBufferConnection, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut direction = None;
                let mut r#type = None;
                let mut connection = None;
                let mut connection_config = None;
                let mut creation_config = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Direction => {
                            if direction.is_some() {
                                return Err(serde::de::Error::duplicate_field("direction"));
                            }
                            direction = Some(map.next_value::<write_buffer_connection::Direction>()? as i32);
                        }
                        GeneratedField::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map.next_value()?);
                        }
                        GeneratedField::Connection => {
                            if connection.is_some() {
                                return Err(serde::de::Error::duplicate_field("connection"));
                            }
                            connection = Some(map.next_value()?);
                        }
                        GeneratedField::ConnectionConfig => {
                            if connection_config.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionConfig"));
                            }
                            connection_config = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::CreationConfig => {
                            if creation_config.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationConfig"));
                            }
                            creation_config = Some(map.next_value()?);
                        }
                    }
                }
                Ok(WriteBufferConnection {
                    direction: direction.unwrap_or_default(),
                    r#type: r#type.unwrap_or_default(),
                    connection: connection.unwrap_or_default(),
                    connection_config: connection_config.unwrap_or_default(),
                    creation_config,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.WriteBufferConnection", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for write_buffer_connection::Direction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "DIRECTION_UNSPECIFIED",
            Self::Write => "DIRECTION_WRITE",
            Self::Read => "DIRECTION_READ",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for write_buffer_connection::Direction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DIRECTION_UNSPECIFIED",
            "DIRECTION_WRITE",
            "DIRECTION_READ",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = write_buffer_connection::Direction;

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
                    .and_then(write_buffer_connection::Direction::from_i32)
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
                    .and_then(write_buffer_connection::Direction::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DIRECTION_UNSPECIFIED" => Ok(write_buffer_connection::Direction::Unspecified),
                    "DIRECTION_WRITE" => Ok(write_buffer_connection::Direction::Write),
                    "DIRECTION_READ" => Ok(write_buffer_connection::Direction::Read),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for WriteBufferCreationConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.n_sequencers != 0 {
            len += 1;
        }
        if !self.options.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.WriteBufferCreationConfig", len)?;
        if self.n_sequencers != 0 {
            struct_ser.serialize_field("nSequencers", &self.n_sequencers)?;
        }
        if !self.options.is_empty() {
            struct_ser.serialize_field("options", &self.options)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteBufferCreationConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nSequencers",
            "options",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NSequencers,
            Options,
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
                            "nSequencers" => Ok(GeneratedField::NSequencers),
                            "options" => Ok(GeneratedField::Options),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteBufferCreationConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.WriteBufferCreationConfig")
            }

            fn visit_map<V>(self, mut map: V) -> Result<WriteBufferCreationConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut n_sequencers = None;
                let mut options = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::NSequencers => {
                            if n_sequencers.is_some() {
                                return Err(serde::de::Error::duplicate_field("nSequencers"));
                            }
                            n_sequencers = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Options => {
                            if options.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(WriteBufferCreationConfig {
                    n_sequencers: n_sequencers.unwrap_or_default(),
                    options: options.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.WriteBufferCreationConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteChunk {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.db_name.is_empty() {
            len += 1;
        }
        if !self.partition_key.is_empty() {
            len += 1;
        }
        if !self.table_name.is_empty() {
            len += 1;
        }
        if self.chunk_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.management.v1.WriteChunk", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        if !self.partition_key.is_empty() {
            struct_ser.serialize_field("partitionKey", &self.partition_key)?;
        }
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        if self.chunk_id != 0 {
            struct_ser.serialize_field("chunkId", &self.chunk_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteChunk {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
            "partitionKey",
            "tableName",
            "chunkId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
            PartitionKey,
            TableName,
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
                            "dbName" => Ok(GeneratedField::DbName),
                            "partitionKey" => Ok(GeneratedField::PartitionKey),
                            "tableName" => Ok(GeneratedField::TableName),
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
            type Value = WriteChunk;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.management.v1.WriteChunk")
            }

            fn visit_map<V>(self, mut map: V) -> Result<WriteChunk, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                let mut partition_key = None;
                let mut table_name = None;
                let mut chunk_id = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                        GeneratedField::PartitionKey => {
                            if partition_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionKey"));
                            }
                            partition_key = Some(map.next_value()?);
                        }
                        GeneratedField::TableName => {
                            if table_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name = Some(map.next_value()?);
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
                Ok(WriteChunk {
                    db_name: db_name.unwrap_or_default(),
                    partition_key: partition_key.unwrap_or_default(),
                    table_name: table_name.unwrap_or_default(),
                    chunk_id: chunk_id.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.management.v1.WriteChunk", FIELDS, GeneratedVisitor)
    }
}
