impl serde::Serialize for BadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.field_violations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.BadRequest", len)?;
        if !self.field_violations.is_empty() {
            struct_ser.serialize_field("fieldViolations", &self.field_violations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fieldViolations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FieldViolations,
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
                            "fieldViolations" => Ok(GeneratedField::FieldViolations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.BadRequest")
            }

            fn visit_map<V>(self, mut map: V) -> Result<BadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field_violations = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FieldViolations => {
                            if field_violations.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldViolations"));
                            }
                            field_violations = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BadRequest {
                    field_violations: field_violations.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.BadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for bad_request::FieldViolation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.field.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.BadRequest.FieldViolation", len)?;
        if !self.field.is_empty() {
            struct_ser.serialize_field("field", &self.field)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for bad_request::FieldViolation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Field,
            Description,
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
                            "field" => Ok(GeneratedField::Field),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = bad_request::FieldViolation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.BadRequest.FieldViolation")
            }

            fn visit_map<V>(self, mut map: V) -> Result<bad_request::FieldViolation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field = None;
                let mut description = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Field => {
                            if field.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        }
                    }
                }
                Ok(bad_request::FieldViolation {
                    field: field.unwrap_or_default(),
                    description: description.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.BadRequest.FieldViolation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DebugInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stack_entries.is_empty() {
            len += 1;
        }
        if !self.detail.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.DebugInfo", len)?;
        if !self.stack_entries.is_empty() {
            struct_ser.serialize_field("stackEntries", &self.stack_entries)?;
        }
        if !self.detail.is_empty() {
            struct_ser.serialize_field("detail", &self.detail)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DebugInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stackEntries",
            "detail",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StackEntries,
            Detail,
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
                            "stackEntries" => Ok(GeneratedField::StackEntries),
                            "detail" => Ok(GeneratedField::Detail),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DebugInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.DebugInfo")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DebugInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stack_entries = None;
                let mut detail = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StackEntries => {
                            if stack_entries.is_some() {
                                return Err(serde::de::Error::duplicate_field("stackEntries"));
                            }
                            stack_entries = Some(map.next_value()?);
                        }
                        GeneratedField::Detail => {
                            if detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("detail"));
                            }
                            detail = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DebugInfo {
                    stack_entries: stack_entries.unwrap_or_default(),
                    detail: detail.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.DebugInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reason.is_empty() {
            len += 1;
        }
        if !self.domain.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.ErrorInfo", len)?;
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ErrorInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reason",
            "domain",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reason,
            Domain,
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
                            "reason" => Ok(GeneratedField::Reason),
                            "domain" => Ok(GeneratedField::Domain),
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
            type Value = ErrorInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.ErrorInfo")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ErrorInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason = None;
                let mut domain = None;
                let mut metadata = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Reason => {
                            if reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason = Some(map.next_value()?);
                        }
                        GeneratedField::Domain => {
                            if domain.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(ErrorInfo {
                    reason: reason.unwrap_or_default(),
                    domain: domain.unwrap_or_default(),
                    metadata: metadata.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.ErrorInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Help {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.links.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.Help", len)?;
        if !self.links.is_empty() {
            struct_ser.serialize_field("links", &self.links)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Help {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "links",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Links,
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
                            "links" => Ok(GeneratedField::Links),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Help;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.Help")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Help, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut links = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Links => {
                            if links.is_some() {
                                return Err(serde::de::Error::duplicate_field("links"));
                            }
                            links = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Help {
                    links: links.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.Help", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for help::Link {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.Help.Link", len)?;
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for help::Link {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
            "url",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
            Url,
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
                            "description" => Ok(GeneratedField::Description),
                            "url" => Ok(GeneratedField::Url),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = help::Link;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.Help.Link")
            }

            fn visit_map<V>(self, mut map: V) -> Result<help::Link, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description = None;
                let mut url = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        }
                        GeneratedField::Url => {
                            if url.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url = Some(map.next_value()?);
                        }
                    }
                }
                Ok(help::Link {
                    description: description.unwrap_or_default(),
                    url: url.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.Help.Link", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LocalizedMessage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.locale.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.LocalizedMessage", len)?;
        if !self.locale.is_empty() {
            struct_ser.serialize_field("locale", &self.locale)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocalizedMessage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "locale",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Locale,
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
                            "locale" => Ok(GeneratedField::Locale),
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
            type Value = LocalizedMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.LocalizedMessage")
            }

            fn visit_map<V>(self, mut map: V) -> Result<LocalizedMessage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut locale = None;
                let mut message = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Locale => {
                            if locale.is_some() {
                                return Err(serde::de::Error::duplicate_field("locale"));
                            }
                            locale = Some(map.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message = Some(map.next_value()?);
                        }
                    }
                }
                Ok(LocalizedMessage {
                    locale: locale.unwrap_or_default(),
                    message: message.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.LocalizedMessage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PreconditionFailure {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.violations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.PreconditionFailure", len)?;
        if !self.violations.is_empty() {
            struct_ser.serialize_field("violations", &self.violations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PreconditionFailure {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "violations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Violations,
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
                            "violations" => Ok(GeneratedField::Violations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PreconditionFailure;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.PreconditionFailure")
            }

            fn visit_map<V>(self, mut map: V) -> Result<PreconditionFailure, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut violations = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Violations => {
                            if violations.is_some() {
                                return Err(serde::de::Error::duplicate_field("violations"));
                            }
                            violations = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PreconditionFailure {
                    violations: violations.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.PreconditionFailure", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for precondition_failure::Violation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.subject.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.PreconditionFailure.Violation", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.subject.is_empty() {
            struct_ser.serialize_field("subject", &self.subject)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for precondition_failure::Violation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "subject",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Subject,
            Description,
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
                            "subject" => Ok(GeneratedField::Subject),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = precondition_failure::Violation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.PreconditionFailure.Violation")
            }

            fn visit_map<V>(self, mut map: V) -> Result<precondition_failure::Violation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type = None;
                let mut subject = None;
                let mut description = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map.next_value()?);
                        }
                        GeneratedField::Subject => {
                            if subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        }
                    }
                }
                Ok(precondition_failure::Violation {
                    r#type: r#type.unwrap_or_default(),
                    subject: subject.unwrap_or_default(),
                    description: description.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.PreconditionFailure.Violation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaFailure {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.violations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.QuotaFailure", len)?;
        if !self.violations.is_empty() {
            struct_ser.serialize_field("violations", &self.violations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaFailure {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "violations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Violations,
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
                            "violations" => Ok(GeneratedField::Violations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaFailure;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.QuotaFailure")
            }

            fn visit_map<V>(self, mut map: V) -> Result<QuotaFailure, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut violations = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Violations => {
                            if violations.is_some() {
                                return Err(serde::de::Error::duplicate_field("violations"));
                            }
                            violations = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QuotaFailure {
                    violations: violations.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.QuotaFailure", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for quota_failure::Violation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subject.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.QuotaFailure.Violation", len)?;
        if !self.subject.is_empty() {
            struct_ser.serialize_field("subject", &self.subject)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for quota_failure::Violation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subject",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subject,
            Description,
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
                            "subject" => Ok(GeneratedField::Subject),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = quota_failure::Violation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.QuotaFailure.Violation")
            }

            fn visit_map<V>(self, mut map: V) -> Result<quota_failure::Violation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subject = None;
                let mut description = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Subject => {
                            if subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        }
                    }
                }
                Ok(quota_failure::Violation {
                    subject: subject.unwrap_or_default(),
                    description: description.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.QuotaFailure.Violation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_id.is_empty() {
            len += 1;
        }
        if !self.serving_data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.RequestInfo", len)?;
        if !self.request_id.is_empty() {
            struct_ser.serialize_field("requestId", &self.request_id)?;
        }
        if !self.serving_data.is_empty() {
            struct_ser.serialize_field("servingData", &self.serving_data)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requestId",
            "servingData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestId,
            ServingData,
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
                            "requestId" => Ok(GeneratedField::RequestId),
                            "servingData" => Ok(GeneratedField::ServingData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.RequestInfo")
            }

            fn visit_map<V>(self, mut map: V) -> Result<RequestInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_id = None;
                let mut serving_data = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RequestId => {
                            if request_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id = Some(map.next_value()?);
                        }
                        GeneratedField::ServingData => {
                            if serving_data.is_some() {
                                return Err(serde::de::Error::duplicate_field("servingData"));
                            }
                            serving_data = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RequestInfo {
                    request_id: request_id.unwrap_or_default(),
                    serving_data: serving_data.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.RequestInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_type.is_empty() {
            len += 1;
        }
        if !self.resource_name.is_empty() {
            len += 1;
        }
        if !self.owner.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.ResourceInfo", len)?;
        if !self.resource_type.is_empty() {
            struct_ser.serialize_field("resourceType", &self.resource_type)?;
        }
        if !self.resource_name.is_empty() {
            struct_ser.serialize_field("resourceName", &self.resource_name)?;
        }
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resourceType",
            "resourceName",
            "owner",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceType,
            ResourceName,
            Owner,
            Description,
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
                            "resourceType" => Ok(GeneratedField::ResourceType),
                            "resourceName" => Ok(GeneratedField::ResourceName),
                            "owner" => Ok(GeneratedField::Owner),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.ResourceInfo")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ResourceInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_type = None;
                let mut resource_name = None;
                let mut owner = None;
                let mut description = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResourceType => {
                            if resource_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceType"));
                            }
                            resource_type = Some(map.next_value()?);
                        }
                        GeneratedField::ResourceName => {
                            if resource_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceName"));
                            }
                            resource_name = Some(map.next_value()?);
                        }
                        GeneratedField::Owner => {
                            if owner.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ResourceInfo {
                    resource_type: resource_type.unwrap_or_default(),
                    resource_name: resource_name.unwrap_or_default(),
                    owner: owner.unwrap_or_default(),
                    description: description.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.ResourceInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RetryInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.retry_delay.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.RetryInfo", len)?;
        if let Some(v) = self.retry_delay.as_ref() {
            struct_ser.serialize_field("retryDelay", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RetryInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "retryDelay",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RetryDelay,
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
                            "retryDelay" => Ok(GeneratedField::RetryDelay),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RetryInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.RetryInfo")
            }

            fn visit_map<V>(self, mut map: V) -> Result<RetryInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut retry_delay = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RetryDelay => {
                            if retry_delay.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryDelay"));
                            }
                            retry_delay = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RetryInfo {
                    retry_delay,
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.RetryInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Status {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if !self.details.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("google.rpc.Status", len)?;
        if self.code != 0 {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if !self.details.is_empty() {
            struct_ser.serialize_field("details", &self.details)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Status {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "message",
            "details",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Message,
            Details,
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
                            "code" => Ok(GeneratedField::Code),
                            "message" => Ok(GeneratedField::Message),
                            "details" => Ok(GeneratedField::Details),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Status;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct google.rpc.Status")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Status, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code = None;
                let mut message = None;
                let mut details = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::Message => {
                            if message.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message = Some(map.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Status {
                    code: code.unwrap_or_default(),
                    message: message.unwrap_or_default(),
                    details: details.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("google.rpc.Status", FIELDS, GeneratedVisitor)
    }
}
