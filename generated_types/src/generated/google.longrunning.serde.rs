impl serde::Serialize for CancelOperationRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.longrunning.CancelOperationRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelOperationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CancelOperationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.longrunning.CancelOperationRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CancelOperationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CancelOperationRequest {
                    name: name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.longrunning.CancelOperationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteOperationRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.longrunning.DeleteOperationRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteOperationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteOperationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.longrunning.DeleteOperationRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DeleteOperationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DeleteOperationRequest {
                    name: name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.longrunning.DeleteOperationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetOperationRequest {
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
        let mut struct_ser = serializer.serialize_struct("google.longrunning.GetOperationRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetOperationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetOperationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.longrunning.GetOperationRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GetOperationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetOperationRequest {
                    name: name.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.longrunning.GetOperationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListOperationsRequest {
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
        if !self.filter.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.longrunning.ListOperationsRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListOperationsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "filter",
            "pageSize",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Filter,
            PageSize,
            PageToken,
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
                            "filter" => Ok(GeneratedField::Filter),
                            "pageSize" => Ok(GeneratedField::PageSize),
                            "pageToken" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListOperationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.longrunning.ListOperationsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ListOperationsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name = None;
                let mut filter = None;
                let mut page_size = None;
                let mut page_token = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        GeneratedField::Filter => {
                            if filter.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter = Some(map.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::PageToken => {
                            if page_token.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ListOperationsRequest {
                    name: name.unwrap_or_default(),
                    filter: filter.unwrap_or_default(),
                    page_size: page_size.unwrap_or_default(),
                    page_token: page_token.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.longrunning.ListOperationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListOperationsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.operations.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.longrunning.ListOperationsResponse", len)?;
        if !self.operations.is_empty() {
            struct_ser.serialize_field("operations", &self.operations)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListOperationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operations",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operations,
            NextPageToken,
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
                            "operations" => Ok(GeneratedField::Operations),
                            "nextPageToken" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListOperationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.longrunning.ListOperationsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ListOperationsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operations = None;
                let mut next_page_token = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Operations => {
                            if operations.is_some() {
                                return Err(serde::de::Error::duplicate_field("operations"));
                            }
                            operations = Some(map.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ListOperationsResponse {
                    operations: operations.unwrap_or_default(),
                    next_page_token: next_page_token.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.longrunning.ListOperationsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Operation {
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
        if self.metadata.is_some() {
            len += 1;
        }
        if self.done {
            len += 1;
        }
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.longrunning.Operation", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if self.done {
            struct_ser.serialize_field("done", &self.done)?;
        }
        if let Some(v) = self.result.as_ref() {
            match v {
                operation::Result::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
                operation::Result::Response(v) => {
                    struct_ser.serialize_field("response", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Operation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "metadata",
            "done",
            "error",
            "response",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Metadata,
            Done,
            Error,
            Response,
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
                            "metadata" => Ok(GeneratedField::Metadata),
                            "done" => Ok(GeneratedField::Done),
                            "error" => Ok(GeneratedField::Error),
                            "response" => Ok(GeneratedField::Response),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Operation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.longrunning.Operation")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Operation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name = None;
                let mut metadata = None;
                let mut done = None;
                let mut result = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata = Some(map.next_value()?);
                        }
                        GeneratedField::Done => {
                            if done.is_some() {
                                return Err(serde::de::Error::duplicate_field("done"));
                            }
                            done = Some(map.next_value()?);
                        }
                        GeneratedField::Error => {
                            if result.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            result = Some(operation::Result::Error(map.next_value()?));
                        }
                        GeneratedField::Response => {
                            if result.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            result = Some(operation::Result::Response(map.next_value()?));
                        }
                    }
                }
                Ok(Operation {
                    name: name.unwrap_or_default(),
                    metadata,
                    done: done.unwrap_or_default(),
                    result,
                })
            }
        }
        deserializer.deserialize_struct("google.longrunning.Operation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OperationInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.response_type.is_empty() {
            len += 1;
        }
        if !self.metadata_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.longrunning.OperationInfo", len)?;
        if !self.response_type.is_empty() {
            struct_ser.serialize_field("responseType", &self.response_type)?;
        }
        if !self.metadata_type.is_empty() {
            struct_ser.serialize_field("metadataType", &self.metadata_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OperationInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "responseType",
            "metadataType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResponseType,
            MetadataType,
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
                            "responseType" => Ok(GeneratedField::ResponseType),
                            "metadataType" => Ok(GeneratedField::MetadataType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OperationInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.longrunning.OperationInfo")
            }

            fn visit_map<V>(self, mut map: V) -> Result<OperationInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response_type = None;
                let mut metadata_type = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResponseType => {
                            if response_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseType"));
                            }
                            response_type = Some(map.next_value()?);
                        }
                        GeneratedField::MetadataType => {
                            if metadata_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataType"));
                            }
                            metadata_type = Some(map.next_value()?);
                        }
                    }
                }
                Ok(OperationInfo {
                    response_type: response_type.unwrap_or_default(),
                    metadata_type: metadata_type.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.longrunning.OperationInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WaitOperationRequest {
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
        if self.timeout.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.longrunning.WaitOperationRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WaitOperationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "timeout",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Timeout,
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
                            "timeout" => Ok(GeneratedField::Timeout),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WaitOperationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.longrunning.WaitOperationRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<WaitOperationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name = None;
                let mut timeout = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        GeneratedField::Timeout => {
                            if timeout.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout = Some(map.next_value()?);
                        }
                    }
                }
                Ok(WaitOperationRequest {
                    name: name.unwrap_or_default(),
                    timeout,
                })
            }
        }
        deserializer.deserialize_struct("google.longrunning.WaitOperationRequest", FIELDS, GeneratedVisitor)
    }
}
