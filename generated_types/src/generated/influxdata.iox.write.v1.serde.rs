impl serde::Serialize for WriteEntryRequest {
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
        if !self.entry.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.write.v1.WriteEntryRequest", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        if !self.entry.is_empty() {
            struct_ser.serialize_field("entry", pbjson::private::base64::encode(&self.entry).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteEntryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dbName",
            "entry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
            Entry,
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
                            "entry" => Ok(GeneratedField::Entry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteEntryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.write.v1.WriteEntryRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<WriteEntryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                let mut entry = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                        GeneratedField::Entry => {
                            if entry.is_some() {
                                return Err(serde::de::Error::duplicate_field("entry"));
                            }
                            entry = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(WriteEntryRequest {
                    db_name: db_name.unwrap_or_default(),
                    entry: entry.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.write.v1.WriteEntryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteEntryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.iox.write.v1.WriteEntryResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteEntryResponse {
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
            type Value = WriteEntryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.write.v1.WriteEntryResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<WriteEntryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(WriteEntryResponse {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.write.v1.WriteEntryResponse", FIELDS, GeneratedVisitor)
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
        if !self.db_name.is_empty() {
            len += 1;
        }
        if !self.lp_data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.write.v1.WriteRequest", len)?;
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        if !self.lp_data.is_empty() {
            struct_ser.serialize_field("lpData", &self.lp_data)?;
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
            "dbName",
            "lpData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DbName,
            LpData,
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
                            "lpData" => Ok(GeneratedField::LpData),
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
                formatter.write_str("struct influxdata.iox.write.v1.WriteRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<WriteRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut db_name = None;
                let mut lp_data = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DbName => {
                            if db_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name = Some(map.next_value()?);
                        }
                        GeneratedField::LpData => {
                            if lp_data.is_some() {
                                return Err(serde::de::Error::duplicate_field("lpData"));
                            }
                            lp_data = Some(map.next_value()?);
                        }
                    }
                }
                Ok(WriteRequest {
                    db_name: db_name.unwrap_or_default(),
                    lp_data: lp_data.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.write.v1.WriteRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.lines_written != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.iox.write.v1.WriteResponse", len)?;
        if self.lines_written != 0 {
            struct_ser.serialize_field("linesWritten", ToString::to_string(&self.lines_written).as_str())?;
        }
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
            "linesWritten",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LinesWritten,
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
                            "linesWritten" => Ok(GeneratedField::LinesWritten),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.iox.write.v1.WriteResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<WriteResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut lines_written = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LinesWritten => {
                            if lines_written.is_some() {
                                return Err(serde::de::Error::duplicate_field("linesWritten"));
                            }
                            lines_written = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(WriteResponse {
                    lines_written: lines_written.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.iox.write.v1.WriteResponse", FIELDS, GeneratedVisitor)
    }
}
