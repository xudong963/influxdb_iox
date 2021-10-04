impl serde::Serialize for Aggregate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.Aggregate", len)?;
        if self.r#type != 0 {
            let v = aggregate::AggregateType::from_i32(self.r#type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Aggregate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
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
                            "type" => Ok(GeneratedField::Type),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Aggregate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.Aggregate")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Aggregate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map.next_value::<aggregate::AggregateType>()? as i32);
                        }
                    }
                }
                Ok(Aggregate {
                    r#type: r#type.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.Aggregate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for aggregate::AggregateType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::None => "NONE",
            Self::Sum => "SUM",
            Self::Count => "COUNT",
            Self::Min => "MIN",
            Self::Max => "MAX",
            Self::First => "FIRST",
            Self::Last => "LAST",
            Self::Mean => "MEAN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for aggregate::AggregateType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NONE",
            "SUM",
            "COUNT",
            "MIN",
            "MAX",
            "FIRST",
            "LAST",
            "MEAN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = aggregate::AggregateType;

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
                    .and_then(aggregate::AggregateType::from_i32)
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
                    .and_then(aggregate::AggregateType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NONE" => Ok(aggregate::AggregateType::None),
                    "SUM" => Ok(aggregate::AggregateType::Sum),
                    "COUNT" => Ok(aggregate::AggregateType::Count),
                    "MIN" => Ok(aggregate::AggregateType::Min),
                    "MAX" => Ok(aggregate::AggregateType::Max),
                    "FIRST" => Ok(aggregate::AggregateType::First),
                    "LAST" => Ok(aggregate::AggregateType::Last),
                    "MEAN" => Ok(aggregate::AggregateType::Mean),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CapabilitiesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.caps.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.CapabilitiesResponse", len)?;
        if !self.caps.is_empty() {
            struct_ser.serialize_field("caps", &self.caps)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "caps",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Caps,
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
                            "caps" => Ok(GeneratedField::Caps),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CapabilitiesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.CapabilitiesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CapabilitiesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut caps = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Caps => {
                            if caps.is_some() {
                                return Err(serde::de::Error::duplicate_field("caps"));
                            }
                            caps = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(CapabilitiesResponse {
                    caps: caps.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.CapabilitiesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Capability {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.features.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.Capability", len)?;
        if !self.features.is_empty() {
            struct_ser.serialize_field("features", &self.features)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Capability {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "features",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Features,
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
                            "features" => Ok(GeneratedField::Features),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Capability;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.Capability")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Capability, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut features = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Features => {
                            if features.is_some() {
                                return Err(serde::de::Error::duplicate_field("features"));
                            }
                            features = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Capability {
                    features: features.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.Capability", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Duration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.nsecs != 0 {
            len += 1;
        }
        if self.months != 0 {
            len += 1;
        }
        if self.negative {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.Duration", len)?;
        if self.nsecs != 0 {
            struct_ser.serialize_field("nsecs", ToString::to_string(&self.nsecs).as_str())?;
        }
        if self.months != 0 {
            struct_ser.serialize_field("months", ToString::to_string(&self.months).as_str())?;
        }
        if self.negative {
            struct_ser.serialize_field("negative", &self.negative)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Duration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nsecs",
            "months",
            "negative",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nsecs,
            Months,
            Negative,
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
                            "nsecs" => Ok(GeneratedField::Nsecs),
                            "months" => Ok(GeneratedField::Months),
                            "negative" => Ok(GeneratedField::Negative),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Duration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.Duration")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Duration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut nsecs = None;
                let mut months = None;
                let mut negative = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Nsecs => {
                            if nsecs.is_some() {
                                return Err(serde::de::Error::duplicate_field("nsecs"));
                            }
                            nsecs = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Months => {
                            if months.is_some() {
                                return Err(serde::de::Error::duplicate_field("months"));
                            }
                            months = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Negative => {
                            if negative.is_some() {
                                return Err(serde::de::Error::duplicate_field("negative"));
                            }
                            negative = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Duration {
                    nsecs: nsecs.unwrap_or_default(),
                    months: months.unwrap_or_default(),
                    negative: negative.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.Duration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Int64ValuesResponse {
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
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.Int64ValuesResponse", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Int64ValuesResponse {
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
            type Value = Int64ValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.Int64ValuesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Int64ValuesResponse, V::Error>
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
                            values = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                    }
                }
                Ok(Int64ValuesResponse {
                    values: values.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.Int64ValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MeasurementFieldsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source.is_some() {
            len += 1;
        }
        if !self.measurement.is_empty() {
            len += 1;
        }
        if self.range.is_some() {
            len += 1;
        }
        if self.predicate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.MeasurementFieldsRequest", len)?;
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if !self.measurement.is_empty() {
            struct_ser.serialize_field("measurement", &self.measurement)?;
        }
        if let Some(v) = self.range.as_ref() {
            struct_ser.serialize_field("range", v)?;
        }
        if let Some(v) = self.predicate.as_ref() {
            struct_ser.serialize_field("predicate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MeasurementFieldsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source",
            "measurement",
            "range",
            "predicate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Source,
            Measurement,
            Range,
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
                            "source" => Ok(GeneratedField::Source),
                            "measurement" => Ok(GeneratedField::Measurement),
                            "range" => Ok(GeneratedField::Range),
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
            type Value = MeasurementFieldsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.MeasurementFieldsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<MeasurementFieldsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source = None;
                let mut measurement = None;
                let mut range = None;
                let mut predicate = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Source => {
                            if source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source = Some(map.next_value()?);
                        }
                        GeneratedField::Measurement => {
                            if measurement.is_some() {
                                return Err(serde::de::Error::duplicate_field("measurement"));
                            }
                            measurement = Some(map.next_value()?);
                        }
                        GeneratedField::Range => {
                            if range.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range = Some(map.next_value()?);
                        }
                        GeneratedField::Predicate => {
                            if predicate.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MeasurementFieldsRequest {
                    source,
                    measurement: measurement.unwrap_or_default(),
                    range,
                    predicate,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.MeasurementFieldsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MeasurementFieldsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fields.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.MeasurementFieldsResponse", len)?;
        if !self.fields.is_empty() {
            struct_ser.serialize_field("fields", &self.fields)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MeasurementFieldsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fields",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fields,
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
                            "fields" => Ok(GeneratedField::Fields),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MeasurementFieldsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.MeasurementFieldsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<MeasurementFieldsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fields = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Fields => {
                            if fields.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            fields = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MeasurementFieldsResponse {
                    fields: fields.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.MeasurementFieldsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for measurement_fields_response::FieldType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Float => "FLOAT",
            Self::Integer => "INTEGER",
            Self::Unsigned => "UNSIGNED",
            Self::String => "STRING",
            Self::Boolean => "BOOLEAN",
            Self::Undefined => "UNDEFINED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for measurement_fields_response::FieldType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FLOAT",
            "INTEGER",
            "UNSIGNED",
            "STRING",
            "BOOLEAN",
            "UNDEFINED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = measurement_fields_response::FieldType;

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
                    .and_then(measurement_fields_response::FieldType::from_i32)
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
                    .and_then(measurement_fields_response::FieldType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "FLOAT" => Ok(measurement_fields_response::FieldType::Float),
                    "INTEGER" => Ok(measurement_fields_response::FieldType::Integer),
                    "UNSIGNED" => Ok(measurement_fields_response::FieldType::Unsigned),
                    "STRING" => Ok(measurement_fields_response::FieldType::String),
                    "BOOLEAN" => Ok(measurement_fields_response::FieldType::Boolean),
                    "UNDEFINED" => Ok(measurement_fields_response::FieldType::Undefined),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for measurement_fields_response::MessageField {
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
        if self.r#type != 0 {
            len += 1;
        }
        if self.timestamp != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.MeasurementFieldsResponse.MessageField", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if self.r#type != 0 {
            let v = measurement_fields_response::FieldType::from_i32(self.r#type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if self.timestamp != 0 {
            struct_ser.serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for measurement_fields_response::MessageField {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "type",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Type,
            Timestamp,
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
                            "type" => Ok(GeneratedField::Type),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = measurement_fields_response::MessageField;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.MeasurementFieldsResponse.MessageField")
            }

            fn visit_map<V>(self, mut map: V) -> Result<measurement_fields_response::MessageField, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key = None;
                let mut r#type = None;
                let mut timestamp = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key = Some(map.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map.next_value::<measurement_fields_response::FieldType>()? as i32);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(measurement_fields_response::MessageField {
                    key: key.unwrap_or_default(),
                    r#type: r#type.unwrap_or_default(),
                    timestamp: timestamp.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.MeasurementFieldsResponse.MessageField", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MeasurementNamesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source.is_some() {
            len += 1;
        }
        if self.range.is_some() {
            len += 1;
        }
        if self.predicate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.MeasurementNamesRequest", len)?;
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if let Some(v) = self.range.as_ref() {
            struct_ser.serialize_field("range", v)?;
        }
        if let Some(v) = self.predicate.as_ref() {
            struct_ser.serialize_field("predicate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MeasurementNamesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source",
            "range",
            "predicate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Source,
            Range,
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
                            "source" => Ok(GeneratedField::Source),
                            "range" => Ok(GeneratedField::Range),
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
            type Value = MeasurementNamesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.MeasurementNamesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<MeasurementNamesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source = None;
                let mut range = None;
                let mut predicate = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Source => {
                            if source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source = Some(map.next_value()?);
                        }
                        GeneratedField::Range => {
                            if range.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range = Some(map.next_value()?);
                        }
                        GeneratedField::Predicate => {
                            if predicate.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MeasurementNamesRequest {
                    source,
                    range,
                    predicate,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.MeasurementNamesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MeasurementTagKeysRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source.is_some() {
            len += 1;
        }
        if !self.measurement.is_empty() {
            len += 1;
        }
        if self.range.is_some() {
            len += 1;
        }
        if self.predicate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.MeasurementTagKeysRequest", len)?;
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if !self.measurement.is_empty() {
            struct_ser.serialize_field("measurement", &self.measurement)?;
        }
        if let Some(v) = self.range.as_ref() {
            struct_ser.serialize_field("range", v)?;
        }
        if let Some(v) = self.predicate.as_ref() {
            struct_ser.serialize_field("predicate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MeasurementTagKeysRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source",
            "measurement",
            "range",
            "predicate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Source,
            Measurement,
            Range,
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
                            "source" => Ok(GeneratedField::Source),
                            "measurement" => Ok(GeneratedField::Measurement),
                            "range" => Ok(GeneratedField::Range),
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
            type Value = MeasurementTagKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.MeasurementTagKeysRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<MeasurementTagKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source = None;
                let mut measurement = None;
                let mut range = None;
                let mut predicate = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Source => {
                            if source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source = Some(map.next_value()?);
                        }
                        GeneratedField::Measurement => {
                            if measurement.is_some() {
                                return Err(serde::de::Error::duplicate_field("measurement"));
                            }
                            measurement = Some(map.next_value()?);
                        }
                        GeneratedField::Range => {
                            if range.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range = Some(map.next_value()?);
                        }
                        GeneratedField::Predicate => {
                            if predicate.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MeasurementTagKeysRequest {
                    source,
                    measurement: measurement.unwrap_or_default(),
                    range,
                    predicate,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.MeasurementTagKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MeasurementTagValuesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source.is_some() {
            len += 1;
        }
        if !self.measurement.is_empty() {
            len += 1;
        }
        if !self.tag_key.is_empty() {
            len += 1;
        }
        if self.range.is_some() {
            len += 1;
        }
        if self.predicate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.MeasurementTagValuesRequest", len)?;
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if !self.measurement.is_empty() {
            struct_ser.serialize_field("measurement", &self.measurement)?;
        }
        if !self.tag_key.is_empty() {
            struct_ser.serialize_field("tagKey", &self.tag_key)?;
        }
        if let Some(v) = self.range.as_ref() {
            struct_ser.serialize_field("range", v)?;
        }
        if let Some(v) = self.predicate.as_ref() {
            struct_ser.serialize_field("predicate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MeasurementTagValuesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source",
            "measurement",
            "tagKey",
            "range",
            "predicate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Source,
            Measurement,
            TagKey,
            Range,
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
                            "source" => Ok(GeneratedField::Source),
                            "measurement" => Ok(GeneratedField::Measurement),
                            "tagKey" => Ok(GeneratedField::TagKey),
                            "range" => Ok(GeneratedField::Range),
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
            type Value = MeasurementTagValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.MeasurementTagValuesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<MeasurementTagValuesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source = None;
                let mut measurement = None;
                let mut tag_key = None;
                let mut range = None;
                let mut predicate = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Source => {
                            if source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source = Some(map.next_value()?);
                        }
                        GeneratedField::Measurement => {
                            if measurement.is_some() {
                                return Err(serde::de::Error::duplicate_field("measurement"));
                            }
                            measurement = Some(map.next_value()?);
                        }
                        GeneratedField::TagKey => {
                            if tag_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagKey"));
                            }
                            tag_key = Some(map.next_value()?);
                        }
                        GeneratedField::Range => {
                            if range.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range = Some(map.next_value()?);
                        }
                        GeneratedField::Predicate => {
                            if predicate.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MeasurementTagValuesRequest {
                    source,
                    measurement: measurement.unwrap_or_default(),
                    tag_key: tag_key.unwrap_or_default(),
                    range,
                    predicate,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.MeasurementTagValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Node {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.node_type != 0 {
            len += 1;
        }
        if !self.children.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.Node", len)?;
        if self.node_type != 0 {
            let v = node::Type::from_i32(self.node_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.node_type)))?;
            struct_ser.serialize_field("nodeType", &v)?;
        }
        if !self.children.is_empty() {
            struct_ser.serialize_field("children", &self.children)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                node::Value::StringValue(v) => {
                    struct_ser.serialize_field("stringValue", v)?;
                }
                node::Value::BoolValue(v) => {
                    struct_ser.serialize_field("boolValue", v)?;
                }
                node::Value::IntValue(v) => {
                    struct_ser.serialize_field("intValue", ToString::to_string(&v).as_str())?;
                }
                node::Value::UintValue(v) => {
                    struct_ser.serialize_field("uintValue", ToString::to_string(&v).as_str())?;
                }
                node::Value::FloatValue(v) => {
                    struct_ser.serialize_field("floatValue", v)?;
                }
                node::Value::RegexValue(v) => {
                    struct_ser.serialize_field("regexValue", v)?;
                }
                node::Value::TagRefValue(v) => {
                    struct_ser.serialize_field("tagRefValue", pbjson::private::base64::encode(&v).as_str())?;
                }
                node::Value::FieldRefValue(v) => {
                    struct_ser.serialize_field("fieldRefValue", v)?;
                }
                node::Value::Logical(v) => {
                    let v = node::Logical::from_i32(*v)
                        .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("logical", &v)?;
                }
                node::Value::Comparison(v) => {
                    let v = node::Comparison::from_i32(*v)
                        .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("comparison", &v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Node {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nodeType",
            "children",
            "stringValue",
            "boolValue",
            "intValue",
            "uintValue",
            "floatValue",
            "regexValue",
            "tagRefValue",
            "fieldRefValue",
            "logical",
            "comparison",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NodeType,
            Children,
            StringValue,
            BoolValue,
            IntValue,
            UintValue,
            FloatValue,
            RegexValue,
            TagRefValue,
            FieldRefValue,
            Logical,
            Comparison,
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
                            "nodeType" => Ok(GeneratedField::NodeType),
                            "children" => Ok(GeneratedField::Children),
                            "stringValue" => Ok(GeneratedField::StringValue),
                            "boolValue" => Ok(GeneratedField::BoolValue),
                            "intValue" => Ok(GeneratedField::IntValue),
                            "uintValue" => Ok(GeneratedField::UintValue),
                            "floatValue" => Ok(GeneratedField::FloatValue),
                            "regexValue" => Ok(GeneratedField::RegexValue),
                            "tagRefValue" => Ok(GeneratedField::TagRefValue),
                            "fieldRefValue" => Ok(GeneratedField::FieldRefValue),
                            "logical" => Ok(GeneratedField::Logical),
                            "comparison" => Ok(GeneratedField::Comparison),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Node;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.Node")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Node, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut node_type = None;
                let mut children = None;
                let mut value = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::NodeType => {
                            if node_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeType"));
                            }
                            node_type = Some(map.next_value::<node::Type>()? as i32);
                        }
                        GeneratedField::Children => {
                            if children.is_some() {
                                return Err(serde::de::Error::duplicate_field("children"));
                            }
                            children = Some(map.next_value()?);
                        }
                        GeneratedField::StringValue => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            value = Some(node::Value::StringValue(map.next_value()?));
                        }
                        GeneratedField::BoolValue => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolValue"));
                            }
                            value = Some(node::Value::BoolValue(map.next_value()?));
                        }
                        GeneratedField::IntValue => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("intValue"));
                            }
                            value = Some(node::Value::IntValue(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            ));
                        }
                        GeneratedField::UintValue => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("uintValue"));
                            }
                            value = Some(node::Value::UintValue(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            ));
                        }
                        GeneratedField::FloatValue => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("floatValue"));
                            }
                            value = Some(node::Value::FloatValue(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            ));
                        }
                        GeneratedField::RegexValue => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("regexValue"));
                            }
                            value = Some(node::Value::RegexValue(map.next_value()?));
                        }
                        GeneratedField::TagRefValue => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagRefValue"));
                            }
                            value = Some(node::Value::TagRefValue(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0
                            ));
                        }
                        GeneratedField::FieldRefValue => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldRefValue"));
                            }
                            value = Some(node::Value::FieldRefValue(map.next_value()?));
                        }
                        GeneratedField::Logical => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("logical"));
                            }
                            value = Some(node::Value::Logical(map.next_value::<node::Logical>()? as i32));
                        }
                        GeneratedField::Comparison => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("comparison"));
                            }
                            value = Some(node::Value::Comparison(map.next_value::<node::Comparison>()? as i32));
                        }
                    }
                }
                Ok(Node {
                    node_type: node_type.unwrap_or_default(),
                    children: children.unwrap_or_default(),
                    value,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.Node", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for node::Comparison {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Equal => "EQUAL",
            Self::NotEqual => "NOT_EQUAL",
            Self::StartsWith => "STARTS_WITH",
            Self::Regex => "REGEX",
            Self::NotRegex => "NOT_REGEX",
            Self::Lt => "LT",
            Self::Lte => "LTE",
            Self::Gt => "GT",
            Self::Gte => "GTE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for node::Comparison {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EQUAL",
            "NOT_EQUAL",
            "STARTS_WITH",
            "REGEX",
            "NOT_REGEX",
            "LT",
            "LTE",
            "GT",
            "GTE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = node::Comparison;

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
                    .and_then(node::Comparison::from_i32)
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
                    .and_then(node::Comparison::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "EQUAL" => Ok(node::Comparison::Equal),
                    "NOT_EQUAL" => Ok(node::Comparison::NotEqual),
                    "STARTS_WITH" => Ok(node::Comparison::StartsWith),
                    "REGEX" => Ok(node::Comparison::Regex),
                    "NOT_REGEX" => Ok(node::Comparison::NotRegex),
                    "LT" => Ok(node::Comparison::Lt),
                    "LTE" => Ok(node::Comparison::Lte),
                    "GT" => Ok(node::Comparison::Gt),
                    "GTE" => Ok(node::Comparison::Gte),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for node::Logical {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::And => "AND",
            Self::Or => "OR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for node::Logical {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AND",
            "OR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = node::Logical;

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
                    .and_then(node::Logical::from_i32)
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
                    .and_then(node::Logical::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "AND" => Ok(node::Logical::And),
                    "OR" => Ok(node::Logical::Or),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for node::Type {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::LogicalExpression => "LOGICAL_EXPRESSION",
            Self::ComparisonExpression => "COMPARISON_EXPRESSION",
            Self::ParenExpression => "PAREN_EXPRESSION",
            Self::TagRef => "TAG_REF",
            Self::Literal => "LITERAL",
            Self::FieldRef => "FIELD_REF",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for node::Type {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "LOGICAL_EXPRESSION",
            "COMPARISON_EXPRESSION",
            "PAREN_EXPRESSION",
            "TAG_REF",
            "LITERAL",
            "FIELD_REF",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = node::Type;

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
                    .and_then(node::Type::from_i32)
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
                    .and_then(node::Type::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "LOGICAL_EXPRESSION" => Ok(node::Type::LogicalExpression),
                    "COMPARISON_EXPRESSION" => Ok(node::Type::ComparisonExpression),
                    "PAREN_EXPRESSION" => Ok(node::Type::ParenExpression),
                    "TAG_REF" => Ok(node::Type::TagRef),
                    "LITERAL" => Ok(node::Type::Literal),
                    "FIELD_REF" => Ok(node::Type::FieldRef),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
        if self.root.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.Predicate", len)?;
        if let Some(v) = self.root.as_ref() {
            struct_ser.serialize_field("root", v)?;
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
            "root",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Root,
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
                            "root" => Ok(GeneratedField::Root),
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
                formatter.write_str("struct influxdata.platform.storage.Predicate")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Predicate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut root = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Root => {
                            if root.is_some() {
                                return Err(serde::de::Error::duplicate_field("root"));
                            }
                            root = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Predicate {
                    root,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.Predicate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadFilterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.read_source.is_some() {
            len += 1;
        }
        if self.range.is_some() {
            len += 1;
        }
        if self.predicate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.ReadFilterRequest", len)?;
        if let Some(v) = self.read_source.as_ref() {
            struct_ser.serialize_field("readSource", v)?;
        }
        if let Some(v) = self.range.as_ref() {
            struct_ser.serialize_field("range", v)?;
        }
        if let Some(v) = self.predicate.as_ref() {
            struct_ser.serialize_field("predicate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadFilterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "readSource",
            "range",
            "predicate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReadSource,
            Range,
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
                            "readSource" => Ok(GeneratedField::ReadSource),
                            "range" => Ok(GeneratedField::Range),
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
            type Value = ReadFilterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.ReadFilterRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ReadFilterRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut read_source = None;
                let mut range = None;
                let mut predicate = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ReadSource => {
                            if read_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("readSource"));
                            }
                            read_source = Some(map.next_value()?);
                        }
                        GeneratedField::Range => {
                            if range.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range = Some(map.next_value()?);
                        }
                        GeneratedField::Predicate => {
                            if predicate.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ReadFilterRequest {
                    read_source,
                    range,
                    predicate,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.ReadFilterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadGroupRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.read_source.is_some() {
            len += 1;
        }
        if self.range.is_some() {
            len += 1;
        }
        if self.predicate.is_some() {
            len += 1;
        }
        if !self.group_keys.is_empty() {
            len += 1;
        }
        if self.group != 0 {
            len += 1;
        }
        if self.aggregate.is_some() {
            len += 1;
        }
        if self.hints != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.ReadGroupRequest", len)?;
        if let Some(v) = self.read_source.as_ref() {
            struct_ser.serialize_field("readSource", v)?;
        }
        if let Some(v) = self.range.as_ref() {
            struct_ser.serialize_field("range", v)?;
        }
        if let Some(v) = self.predicate.as_ref() {
            struct_ser.serialize_field("predicate", v)?;
        }
        if !self.group_keys.is_empty() {
            struct_ser.serialize_field("groupKeys", &self.group_keys)?;
        }
        if self.group != 0 {
            let v = read_group_request::Group::from_i32(self.group)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.group)))?;
            struct_ser.serialize_field("group", &v)?;
        }
        if let Some(v) = self.aggregate.as_ref() {
            struct_ser.serialize_field("aggregate", v)?;
        }
        if self.hints != 0 {
            struct_ser.serialize_field("hints", &self.hints)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadGroupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "readSource",
            "range",
            "predicate",
            "groupKeys",
            "group",
            "aggregate",
            "hints",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReadSource,
            Range,
            Predicate,
            GroupKeys,
            Group,
            Aggregate,
            Hints,
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
                            "readSource" => Ok(GeneratedField::ReadSource),
                            "range" => Ok(GeneratedField::Range),
                            "predicate" => Ok(GeneratedField::Predicate),
                            "groupKeys" => Ok(GeneratedField::GroupKeys),
                            "group" => Ok(GeneratedField::Group),
                            "aggregate" => Ok(GeneratedField::Aggregate),
                            "hints" => Ok(GeneratedField::Hints),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.ReadGroupRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ReadGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut read_source = None;
                let mut range = None;
                let mut predicate = None;
                let mut group_keys = None;
                let mut group = None;
                let mut aggregate = None;
                let mut hints = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ReadSource => {
                            if read_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("readSource"));
                            }
                            read_source = Some(map.next_value()?);
                        }
                        GeneratedField::Range => {
                            if range.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range = Some(map.next_value()?);
                        }
                        GeneratedField::Predicate => {
                            if predicate.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate = Some(map.next_value()?);
                        }
                        GeneratedField::GroupKeys => {
                            if group_keys.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupKeys"));
                            }
                            group_keys = Some(map.next_value()?);
                        }
                        GeneratedField::Group => {
                            if group.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group = Some(map.next_value::<read_group_request::Group>()? as i32);
                        }
                        GeneratedField::Aggregate => {
                            if aggregate.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregate"));
                            }
                            aggregate = Some(map.next_value()?);
                        }
                        GeneratedField::Hints => {
                            if hints.is_some() {
                                return Err(serde::de::Error::duplicate_field("hints"));
                            }
                            hints = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(ReadGroupRequest {
                    read_source,
                    range,
                    predicate,
                    group_keys: group_keys.unwrap_or_default(),
                    group: group.unwrap_or_default(),
                    aggregate,
                    hints: hints.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.ReadGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for read_group_request::Group {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::None => "GROUP_NONE",
            Self::By => "GROUP_BY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for read_group_request::Group {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "GROUP_NONE",
            "GROUP_BY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = read_group_request::Group;

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
                    .and_then(read_group_request::Group::from_i32)
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
                    .and_then(read_group_request::Group::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "GROUP_NONE" => Ok(read_group_request::Group::None),
                    "GROUP_BY" => Ok(read_group_request::Group::By),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for read_group_request::HintFlags {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::HintNone => "HINT_NONE",
            Self::HintNoPoints => "HINT_NO_POINTS",
            Self::HintNoSeries => "HINT_NO_SERIES",
            Self::HintSchemaAllTime => "HINT_SCHEMA_ALL_TIME",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for read_group_request::HintFlags {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "HINT_NONE",
            "HINT_NO_POINTS",
            "HINT_NO_SERIES",
            "HINT_SCHEMA_ALL_TIME",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = read_group_request::HintFlags;

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
                    .and_then(read_group_request::HintFlags::from_i32)
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
                    .and_then(read_group_request::HintFlags::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "HINT_NONE" => Ok(read_group_request::HintFlags::HintNone),
                    "HINT_NO_POINTS" => Ok(read_group_request::HintFlags::HintNoPoints),
                    "HINT_NO_SERIES" => Ok(read_group_request::HintFlags::HintNoSeries),
                    "HINT_SCHEMA_ALL_TIME" => Ok(read_group_request::HintFlags::HintSchemaAllTime),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ReadResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.frames.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.ReadResponse", len)?;
        if !self.frames.is_empty() {
            struct_ser.serialize_field("frames", &self.frames)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "frames",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Frames,
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
                            "frames" => Ok(GeneratedField::Frames),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.ReadResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ReadResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut frames = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Frames => {
                            if frames.is_some() {
                                return Err(serde::de::Error::duplicate_field("frames"));
                            }
                            frames = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ReadResponse {
                    frames: frames.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.ReadResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for read_response::BooleanPointsFrame {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.timestamps.is_empty() {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.ReadResponse.BooleanPointsFrame", len)?;
        if !self.timestamps.is_empty() {
            struct_ser.serialize_field("timestamps", &self.timestamps.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for read_response::BooleanPointsFrame {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamps",
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamps,
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
                            "timestamps" => Ok(GeneratedField::Timestamps),
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
            type Value = read_response::BooleanPointsFrame;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.ReadResponse.BooleanPointsFrame")
            }

            fn visit_map<V>(self, mut map: V) -> Result<read_response::BooleanPointsFrame, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamps = None;
                let mut values = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Timestamps => {
                            if timestamps.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamps"));
                            }
                            timestamps = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                        GeneratedField::Values => {
                            if values.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values = Some(map.next_value()?);
                        }
                    }
                }
                Ok(read_response::BooleanPointsFrame {
                    timestamps: timestamps.unwrap_or_default(),
                    values: values.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.ReadResponse.BooleanPointsFrame", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for read_response::DataType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Float => "FLOAT",
            Self::Integer => "INTEGER",
            Self::Unsigned => "UNSIGNED",
            Self::Boolean => "BOOLEAN",
            Self::String => "STRING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for read_response::DataType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FLOAT",
            "INTEGER",
            "UNSIGNED",
            "BOOLEAN",
            "STRING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = read_response::DataType;

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
                    .and_then(read_response::DataType::from_i32)
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
                    .and_then(read_response::DataType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "FLOAT" => Ok(read_response::DataType::Float),
                    "INTEGER" => Ok(read_response::DataType::Integer),
                    "UNSIGNED" => Ok(read_response::DataType::Unsigned),
                    "BOOLEAN" => Ok(read_response::DataType::Boolean),
                    "STRING" => Ok(read_response::DataType::String),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for read_response::FloatPointsFrame {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.timestamps.is_empty() {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.ReadResponse.FloatPointsFrame", len)?;
        if !self.timestamps.is_empty() {
            struct_ser.serialize_field("timestamps", &self.timestamps.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for read_response::FloatPointsFrame {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamps",
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamps,
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
                            "timestamps" => Ok(GeneratedField::Timestamps),
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
            type Value = read_response::FloatPointsFrame;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.ReadResponse.FloatPointsFrame")
            }

            fn visit_map<V>(self, mut map: V) -> Result<read_response::FloatPointsFrame, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamps = None;
                let mut values = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Timestamps => {
                            if timestamps.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamps"));
                            }
                            timestamps = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                        GeneratedField::Values => {
                            if values.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                    }
                }
                Ok(read_response::FloatPointsFrame {
                    timestamps: timestamps.unwrap_or_default(),
                    values: values.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.ReadResponse.FloatPointsFrame", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for read_response::Frame {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.ReadResponse.Frame", len)?;
        if let Some(v) = self.data.as_ref() {
            match v {
                read_response::frame::Data::Group(v) => {
                    struct_ser.serialize_field("group", v)?;
                }
                read_response::frame::Data::Series(v) => {
                    struct_ser.serialize_field("series", v)?;
                }
                read_response::frame::Data::FloatPoints(v) => {
                    struct_ser.serialize_field("floatPoints", v)?;
                }
                read_response::frame::Data::IntegerPoints(v) => {
                    struct_ser.serialize_field("integerPoints", v)?;
                }
                read_response::frame::Data::UnsignedPoints(v) => {
                    struct_ser.serialize_field("unsignedPoints", v)?;
                }
                read_response::frame::Data::BooleanPoints(v) => {
                    struct_ser.serialize_field("booleanPoints", v)?;
                }
                read_response::frame::Data::StringPoints(v) => {
                    struct_ser.serialize_field("stringPoints", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for read_response::Frame {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "group",
            "series",
            "floatPoints",
            "integerPoints",
            "unsignedPoints",
            "booleanPoints",
            "stringPoints",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Group,
            Series,
            FloatPoints,
            IntegerPoints,
            UnsignedPoints,
            BooleanPoints,
            StringPoints,
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
                            "group" => Ok(GeneratedField::Group),
                            "series" => Ok(GeneratedField::Series),
                            "floatPoints" => Ok(GeneratedField::FloatPoints),
                            "integerPoints" => Ok(GeneratedField::IntegerPoints),
                            "unsignedPoints" => Ok(GeneratedField::UnsignedPoints),
                            "booleanPoints" => Ok(GeneratedField::BooleanPoints),
                            "stringPoints" => Ok(GeneratedField::StringPoints),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = read_response::Frame;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.ReadResponse.Frame")
            }

            fn visit_map<V>(self, mut map: V) -> Result<read_response::Frame, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Group => {
                            if data.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            data = Some(read_response::frame::Data::Group(map.next_value()?));
                        }
                        GeneratedField::Series => {
                            if data.is_some() {
                                return Err(serde::de::Error::duplicate_field("series"));
                            }
                            data = Some(read_response::frame::Data::Series(map.next_value()?));
                        }
                        GeneratedField::FloatPoints => {
                            if data.is_some() {
                                return Err(serde::de::Error::duplicate_field("floatPoints"));
                            }
                            data = Some(read_response::frame::Data::FloatPoints(map.next_value()?));
                        }
                        GeneratedField::IntegerPoints => {
                            if data.is_some() {
                                return Err(serde::de::Error::duplicate_field("integerPoints"));
                            }
                            data = Some(read_response::frame::Data::IntegerPoints(map.next_value()?));
                        }
                        GeneratedField::UnsignedPoints => {
                            if data.is_some() {
                                return Err(serde::de::Error::duplicate_field("unsignedPoints"));
                            }
                            data = Some(read_response::frame::Data::UnsignedPoints(map.next_value()?));
                        }
                        GeneratedField::BooleanPoints => {
                            if data.is_some() {
                                return Err(serde::de::Error::duplicate_field("booleanPoints"));
                            }
                            data = Some(read_response::frame::Data::BooleanPoints(map.next_value()?));
                        }
                        GeneratedField::StringPoints => {
                            if data.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringPoints"));
                            }
                            data = Some(read_response::frame::Data::StringPoints(map.next_value()?));
                        }
                    }
                }
                Ok(read_response::Frame {
                    data,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.ReadResponse.Frame", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for read_response::FrameType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Series => "SERIES",
            Self::Points => "POINTS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for read_response::FrameType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SERIES",
            "POINTS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = read_response::FrameType;

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
                    .and_then(read_response::FrameType::from_i32)
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
                    .and_then(read_response::FrameType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SERIES" => Ok(read_response::FrameType::Series),
                    "POINTS" => Ok(read_response::FrameType::Points),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for read_response::GroupFrame {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tag_keys.is_empty() {
            len += 1;
        }
        if !self.partition_key_vals.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.ReadResponse.GroupFrame", len)?;
        if !self.tag_keys.is_empty() {
            struct_ser.serialize_field("tagKeys", &self.tag_keys.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.partition_key_vals.is_empty() {
            struct_ser.serialize_field("partitionKeyVals", &self.partition_key_vals.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for read_response::GroupFrame {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tagKeys",
            "partitionKeyVals",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TagKeys,
            PartitionKeyVals,
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
                            "tagKeys" => Ok(GeneratedField::TagKeys),
                            "partitionKeyVals" => Ok(GeneratedField::PartitionKeyVals),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = read_response::GroupFrame;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.ReadResponse.GroupFrame")
            }

            fn visit_map<V>(self, mut map: V) -> Result<read_response::GroupFrame, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tag_keys = None;
                let mut partition_key_vals = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TagKeys => {
                            if tag_keys.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagKeys"));
                            }
                            tag_keys = Some(
                                map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                        GeneratedField::PartitionKeyVals => {
                            if partition_key_vals.is_some() {
                                return Err(serde::de::Error::duplicate_field("partitionKeyVals"));
                            }
                            partition_key_vals = Some(
                                map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                    }
                }
                Ok(read_response::GroupFrame {
                    tag_keys: tag_keys.unwrap_or_default(),
                    partition_key_vals: partition_key_vals.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.ReadResponse.GroupFrame", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for read_response::IntegerPointsFrame {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.timestamps.is_empty() {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.ReadResponse.IntegerPointsFrame", len)?;
        if !self.timestamps.is_empty() {
            struct_ser.serialize_field("timestamps", &self.timestamps.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for read_response::IntegerPointsFrame {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamps",
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamps,
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
                            "timestamps" => Ok(GeneratedField::Timestamps),
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
            type Value = read_response::IntegerPointsFrame;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.ReadResponse.IntegerPointsFrame")
            }

            fn visit_map<V>(self, mut map: V) -> Result<read_response::IntegerPointsFrame, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamps = None;
                let mut values = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Timestamps => {
                            if timestamps.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamps"));
                            }
                            timestamps = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                        GeneratedField::Values => {
                            if values.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                    }
                }
                Ok(read_response::IntegerPointsFrame {
                    timestamps: timestamps.unwrap_or_default(),
                    values: values.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.ReadResponse.IntegerPointsFrame", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for read_response::SeriesFrame {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tags.is_empty() {
            len += 1;
        }
        if self.data_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.ReadResponse.SeriesFrame", len)?;
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if self.data_type != 0 {
            let v = read_response::DataType::from_i32(self.data_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.data_type)))?;
            struct_ser.serialize_field("dataType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for read_response::SeriesFrame {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tags",
            "dataType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tags,
            DataType,
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
                            "tags" => Ok(GeneratedField::Tags),
                            "dataType" => Ok(GeneratedField::DataType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = read_response::SeriesFrame;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.ReadResponse.SeriesFrame")
            }

            fn visit_map<V>(self, mut map: V) -> Result<read_response::SeriesFrame, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tags = None;
                let mut data_type = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Tags => {
                            if tags.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags = Some(map.next_value()?);
                        }
                        GeneratedField::DataType => {
                            if data_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataType"));
                            }
                            data_type = Some(map.next_value::<read_response::DataType>()? as i32);
                        }
                    }
                }
                Ok(read_response::SeriesFrame {
                    tags: tags.unwrap_or_default(),
                    data_type: data_type.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.ReadResponse.SeriesFrame", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for read_response::StringPointsFrame {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.timestamps.is_empty() {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.ReadResponse.StringPointsFrame", len)?;
        if !self.timestamps.is_empty() {
            struct_ser.serialize_field("timestamps", &self.timestamps.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for read_response::StringPointsFrame {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamps",
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamps,
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
                            "timestamps" => Ok(GeneratedField::Timestamps),
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
            type Value = read_response::StringPointsFrame;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.ReadResponse.StringPointsFrame")
            }

            fn visit_map<V>(self, mut map: V) -> Result<read_response::StringPointsFrame, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamps = None;
                let mut values = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Timestamps => {
                            if timestamps.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamps"));
                            }
                            timestamps = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                        GeneratedField::Values => {
                            if values.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values = Some(map.next_value()?);
                        }
                    }
                }
                Ok(read_response::StringPointsFrame {
                    timestamps: timestamps.unwrap_or_default(),
                    values: values.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.ReadResponse.StringPointsFrame", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for read_response::UnsignedPointsFrame {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.timestamps.is_empty() {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.ReadResponse.UnsignedPointsFrame", len)?;
        if !self.timestamps.is_empty() {
            struct_ser.serialize_field("timestamps", &self.timestamps.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for read_response::UnsignedPointsFrame {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamps",
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamps,
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
                            "timestamps" => Ok(GeneratedField::Timestamps),
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
            type Value = read_response::UnsignedPointsFrame;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.ReadResponse.UnsignedPointsFrame")
            }

            fn visit_map<V>(self, mut map: V) -> Result<read_response::UnsignedPointsFrame, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamps = None;
                let mut values = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Timestamps => {
                            if timestamps.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamps"));
                            }
                            timestamps = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                        GeneratedField::Values => {
                            if values.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values = Some(
                                map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                    }
                }
                Ok(read_response::UnsignedPointsFrame {
                    timestamps: timestamps.unwrap_or_default(),
                    values: values.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.ReadResponse.UnsignedPointsFrame", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadSeriesCardinalityRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.read_series_cardinality_source.is_some() {
            len += 1;
        }
        if self.range.is_some() {
            len += 1;
        }
        if self.predicate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.ReadSeriesCardinalityRequest", len)?;
        if let Some(v) = self.read_series_cardinality_source.as_ref() {
            struct_ser.serialize_field("readSeriesCardinalitySource", v)?;
        }
        if let Some(v) = self.range.as_ref() {
            struct_ser.serialize_field("range", v)?;
        }
        if let Some(v) = self.predicate.as_ref() {
            struct_ser.serialize_field("predicate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadSeriesCardinalityRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "readSeriesCardinalitySource",
            "range",
            "predicate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReadSeriesCardinalitySource,
            Range,
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
                            "readSeriesCardinalitySource" => Ok(GeneratedField::ReadSeriesCardinalitySource),
                            "range" => Ok(GeneratedField::Range),
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
            type Value = ReadSeriesCardinalityRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.ReadSeriesCardinalityRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ReadSeriesCardinalityRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut read_series_cardinality_source = None;
                let mut range = None;
                let mut predicate = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ReadSeriesCardinalitySource => {
                            if read_series_cardinality_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("readSeriesCardinalitySource"));
                            }
                            read_series_cardinality_source = Some(map.next_value()?);
                        }
                        GeneratedField::Range => {
                            if range.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range = Some(map.next_value()?);
                        }
                        GeneratedField::Predicate => {
                            if predicate.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ReadSeriesCardinalityRequest {
                    read_series_cardinality_source,
                    range,
                    predicate,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.ReadSeriesCardinalityRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadWindowAggregateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.read_source.is_some() {
            len += 1;
        }
        if self.range.is_some() {
            len += 1;
        }
        if self.predicate.is_some() {
            len += 1;
        }
        if self.window_every != 0 {
            len += 1;
        }
        if self.offset != 0 {
            len += 1;
        }
        if !self.aggregate.is_empty() {
            len += 1;
        }
        if self.window.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.ReadWindowAggregateRequest", len)?;
        if let Some(v) = self.read_source.as_ref() {
            struct_ser.serialize_field("readSource", v)?;
        }
        if let Some(v) = self.range.as_ref() {
            struct_ser.serialize_field("range", v)?;
        }
        if let Some(v) = self.predicate.as_ref() {
            struct_ser.serialize_field("predicate", v)?;
        }
        if self.window_every != 0 {
            struct_ser.serialize_field("WindowEvery", ToString::to_string(&self.window_every).as_str())?;
        }
        if self.offset != 0 {
            struct_ser.serialize_field("Offset", ToString::to_string(&self.offset).as_str())?;
        }
        if !self.aggregate.is_empty() {
            struct_ser.serialize_field("aggregate", &self.aggregate)?;
        }
        if let Some(v) = self.window.as_ref() {
            struct_ser.serialize_field("window", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadWindowAggregateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "readSource",
            "range",
            "predicate",
            "WindowEvery",
            "Offset",
            "aggregate",
            "window",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReadSource,
            Range,
            Predicate,
            WindowEvery,
            Offset,
            Aggregate,
            Window,
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
                            "readSource" => Ok(GeneratedField::ReadSource),
                            "range" => Ok(GeneratedField::Range),
                            "predicate" => Ok(GeneratedField::Predicate),
                            "WindowEvery" => Ok(GeneratedField::WindowEvery),
                            "Offset" => Ok(GeneratedField::Offset),
                            "aggregate" => Ok(GeneratedField::Aggregate),
                            "window" => Ok(GeneratedField::Window),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadWindowAggregateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.ReadWindowAggregateRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ReadWindowAggregateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut read_source = None;
                let mut range = None;
                let mut predicate = None;
                let mut window_every = None;
                let mut offset = None;
                let mut aggregate = None;
                let mut window = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ReadSource => {
                            if read_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("readSource"));
                            }
                            read_source = Some(map.next_value()?);
                        }
                        GeneratedField::Range => {
                            if range.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range = Some(map.next_value()?);
                        }
                        GeneratedField::Predicate => {
                            if predicate.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate = Some(map.next_value()?);
                        }
                        GeneratedField::WindowEvery => {
                            if window_every.is_some() {
                                return Err(serde::de::Error::duplicate_field("WindowEvery"));
                            }
                            window_every = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Offset => {
                            if offset.is_some() {
                                return Err(serde::de::Error::duplicate_field("Offset"));
                            }
                            offset = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Aggregate => {
                            if aggregate.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregate"));
                            }
                            aggregate = Some(map.next_value()?);
                        }
                        GeneratedField::Window => {
                            if window.is_some() {
                                return Err(serde::de::Error::duplicate_field("window"));
                            }
                            window = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ReadWindowAggregateRequest {
                    read_source,
                    range,
                    predicate,
                    window_every: window_every.unwrap_or_default(),
                    offset: offset.unwrap_or_default(),
                    aggregate: aggregate.unwrap_or_default(),
                    window,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.ReadWindowAggregateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StringValuesResponse {
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
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.StringValuesResponse", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StringValuesResponse {
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
            type Value = StringValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.StringValuesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<StringValuesResponse, V::Error>
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
                            values = Some(
                                map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect()
                            );
                        }
                    }
                }
                Ok(StringValuesResponse {
                    values: values.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.StringValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Tag {
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
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.Tag", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", pbjson::private::base64::encode(&self.key).as_str())?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Tag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
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
                            "key" => Ok(GeneratedField::Key),
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
            type Value = Tag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.Tag")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Tag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key = None;
                let mut value = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Value => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(Tag {
                    key: key.unwrap_or_default(),
                    value: value.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.Tag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TagKeysRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tags_source.is_some() {
            len += 1;
        }
        if self.range.is_some() {
            len += 1;
        }
        if self.predicate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.TagKeysRequest", len)?;
        if let Some(v) = self.tags_source.as_ref() {
            struct_ser.serialize_field("tagsSource", v)?;
        }
        if let Some(v) = self.range.as_ref() {
            struct_ser.serialize_field("range", v)?;
        }
        if let Some(v) = self.predicate.as_ref() {
            struct_ser.serialize_field("predicate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TagKeysRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tagsSource",
            "range",
            "predicate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TagsSource,
            Range,
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
                            "tagsSource" => Ok(GeneratedField::TagsSource),
                            "range" => Ok(GeneratedField::Range),
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
            type Value = TagKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.TagKeysRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<TagKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tags_source = None;
                let mut range = None;
                let mut predicate = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TagsSource => {
                            if tags_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagsSource"));
                            }
                            tags_source = Some(map.next_value()?);
                        }
                        GeneratedField::Range => {
                            if range.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range = Some(map.next_value()?);
                        }
                        GeneratedField::Predicate => {
                            if predicate.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate = Some(map.next_value()?);
                        }
                    }
                }
                Ok(TagKeysRequest {
                    tags_source,
                    range,
                    predicate,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.TagKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TagValuesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tags_source.is_some() {
            len += 1;
        }
        if self.range.is_some() {
            len += 1;
        }
        if self.predicate.is_some() {
            len += 1;
        }
        if !self.tag_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.TagValuesRequest", len)?;
        if let Some(v) = self.tags_source.as_ref() {
            struct_ser.serialize_field("tagsSource", v)?;
        }
        if let Some(v) = self.range.as_ref() {
            struct_ser.serialize_field("range", v)?;
        }
        if let Some(v) = self.predicate.as_ref() {
            struct_ser.serialize_field("predicate", v)?;
        }
        if !self.tag_key.is_empty() {
            struct_ser.serialize_field("tagKey", pbjson::private::base64::encode(&self.tag_key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TagValuesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tagsSource",
            "range",
            "predicate",
            "tagKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TagsSource,
            Range,
            Predicate,
            TagKey,
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
                            "tagsSource" => Ok(GeneratedField::TagsSource),
                            "range" => Ok(GeneratedField::Range),
                            "predicate" => Ok(GeneratedField::Predicate),
                            "tagKey" => Ok(GeneratedField::TagKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TagValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.TagValuesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<TagValuesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tags_source = None;
                let mut range = None;
                let mut predicate = None;
                let mut tag_key = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TagsSource => {
                            if tags_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagsSource"));
                            }
                            tags_source = Some(map.next_value()?);
                        }
                        GeneratedField::Range => {
                            if range.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range = Some(map.next_value()?);
                        }
                        GeneratedField::Predicate => {
                            if predicate.is_some() {
                                return Err(serde::de::Error::duplicate_field("predicate"));
                            }
                            predicate = Some(map.next_value()?);
                        }
                        GeneratedField::TagKey => {
                            if tag_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagKey"));
                            }
                            tag_key = Some(
                                map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(TagValuesRequest {
                    tags_source,
                    range,
                    predicate,
                    tag_key: tag_key.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.TagValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestErrorRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.platform.storage.TestErrorRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestErrorRequest {
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
            type Value = TestErrorRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.TestErrorRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<TestErrorRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(TestErrorRequest {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.TestErrorRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestErrorResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("influxdata.platform.storage.TestErrorResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestErrorResponse {
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
            type Value = TestErrorResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.TestErrorResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<TestErrorResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {}
                Ok(TestErrorResponse {
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.TestErrorResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.TimestampRange", len)?;
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
                formatter.write_str("struct influxdata.platform.storage.TimestampRange")
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
        deserializer.deserialize_struct("influxdata.platform.storage.TimestampRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Window {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.every.is_some() {
            len += 1;
        }
        if self.offset.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("influxdata.platform.storage.Window", len)?;
        if let Some(v) = self.every.as_ref() {
            struct_ser.serialize_field("every", v)?;
        }
        if let Some(v) = self.offset.as_ref() {
            struct_ser.serialize_field("offset", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Window {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "every",
            "offset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Every,
            Offset,
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
                            "every" => Ok(GeneratedField::Every),
                            "offset" => Ok(GeneratedField::Offset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Window;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct influxdata.platform.storage.Window")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Window, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut every = None;
                let mut offset = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Every => {
                            if every.is_some() {
                                return Err(serde::de::Error::duplicate_field("every"));
                            }
                            every = Some(map.next_value()?);
                        }
                        GeneratedField::Offset => {
                            if offset.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            offset = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Window {
                    every,
                    offset,
                })
            }
        }
        deserializer.deserialize_struct("influxdata.platform.storage.Window", FIELDS, GeneratedVisitor)
    }
}
