use std::sync::Arc;

use data_types::{
    database_rules::{DatabaseRules, WriteBufferConnection},
    server_id::ServerId,
};
use uuid::Uuid;

use crate::{
    core::{WriteBufferError, WriteBufferReading, WriteBufferWriting},
    kafka::{KafkaBufferConsumer, KafkaBufferProducer},
    mock::{MockBufferForReading, MockBufferForWriting, MockBufferSharedState},
};

const PREFIX_MOCK: &str = "mock://";

#[derive(Debug)]
pub enum WriteBufferConfig {
    Writing(Arc<dyn WriteBufferWriting>),
    Reading(Arc<tokio::sync::Mutex<Box<dyn WriteBufferReading>>>),
}

impl WriteBufferConfig {
    pub async fn new(
        server_id: ServerId,
        rules: &DatabaseRules,
    ) -> Result<Option<Self>, WriteBufferError> {
        let name = rules.db_name();

        match rules.write_buffer_connection.as_ref() {
            Some(WriteBufferConnection::Writing(conn)) => {
                if let Some(conn) = conn.strip_prefix(PREFIX_MOCK) {
                    let id = Uuid::parse_str(conn)?;
                    let mock_state = MockBufferSharedState::get(id)
                        .ok_or_else::<WriteBufferError, _>(|| {
                            format!("Unknown mock ID: {}", id).into()
                        })?;
                    let mock_buffer = MockBufferForWriting::new(mock_state);

                    Ok(Some(Self::Writing(Arc::new(mock_buffer) as _)))
                } else {
                    let kafka_buffer = KafkaBufferProducer::new(conn, name)?;

                    Ok(Some(Self::Writing(Arc::new(kafka_buffer) as _)))
                }
            }
            Some(WriteBufferConnection::Reading(conn)) => {
                if let Some(conn) = conn.strip_prefix(PREFIX_MOCK) {
                    let id = Uuid::parse_str(conn)?;
                    let mock_state = MockBufferSharedState::get(id)
                        .ok_or_else::<WriteBufferError, _>(|| {
                            format!("Unknown mock ID: {}", id).into()
                        })?;
                    let mock_buffer = MockBufferForReading::new(mock_state);

                    Ok(Some(Self::Reading(Arc::new(tokio::sync::Mutex::new(
                        Box::new(mock_buffer) as _,
                    )))))
                } else {
                    let kafka_buffer = KafkaBufferConsumer::new(conn, server_id, name).await?;

                    Ok(Some(Self::Reading(Arc::new(tokio::sync::Mutex::new(
                        Box::new(kafka_buffer) as _,
                    )))))
                }
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use data_types::DatabaseName;

    use crate::mock::MockBufferSharedState;

    use super::*;

    #[tokio::test]
    async fn test_none() {
        let server_id = ServerId::try_from(1).unwrap();
        let mut rules = DatabaseRules::new(DatabaseName::new("foo").unwrap());
        rules.write_buffer_connection = None;
        assert!(WriteBufferConfig::new(server_id, &rules)
            .await
            .unwrap()
            .is_none());
    }

    #[tokio::test]
    async fn test_writing_kafka() {
        let server_id = ServerId::try_from(1).unwrap();
        let mut rules = DatabaseRules::new(DatabaseName::new("foo").unwrap());
        rules.write_buffer_connection =
            Some(WriteBufferConnection::Writing("127.0.0.1:2".to_string()));
        if let WriteBufferConfig::Writing(conn) = WriteBufferConfig::new(server_id, &rules)
            .await
            .unwrap()
            .unwrap()
        {
            assert_eq!(conn.type_name(), "kafka");
        } else {
            panic!("not a writing connection");
        }
    }

    //  blocks until https://github.com/influxdata/influxdb_iox/issues/2189 is solved
    // #[tokio::test]
    // async fn test_reading_kafka() {
    //     let server_id = ServerId::try_from(1).unwrap();
    //     let mut rules = DatabaseRules::new(DatabaseName::new("foo").unwrap());
    //     rules.write_buffer_connection = Some(WriteBufferConnection::Reading("test".to_string()));
    //     if let WriteBufferConfig::Reading(conn) = WriteBufferConfig::new(server_id, &rules).await.unwrap().unwrap() {
    //         let conn = conn.lock().await;
    //         assert_eq!(conn.type_name(), "kafka");
    //     } else {
    //         panic!("not a reading connection");
    //     }
    // }

    #[tokio::test]
    async fn test_writing_mock() {
        let state = MockBufferSharedState::empty_with_n_sequencers(1);

        let server_id = ServerId::try_from(1).unwrap();
        let mut rules = DatabaseRules::new(DatabaseName::new("foo").unwrap());
        rules.write_buffer_connection = Some(WriteBufferConnection::Writing(format!(
            "mock://{}",
            state.id()
        )));
        if let WriteBufferConfig::Writing(conn) = WriteBufferConfig::new(server_id, &rules)
            .await
            .unwrap()
            .unwrap()
        {
            assert_eq!(conn.type_name(), "mock");
        } else {
            panic!("not a writing connection");
        }

        // will error when state is unknown
        drop(state);
        let err = WriteBufferConfig::new(server_id, &rules).await.unwrap_err();
        assert!(err.to_string().starts_with("Unknown mock ID:"));
    }

    #[tokio::test]
    async fn test_reading_mock() {
        let state = MockBufferSharedState::empty_with_n_sequencers(1);

        let server_id = ServerId::try_from(1).unwrap();
        let mut rules = DatabaseRules::new(DatabaseName::new("foo").unwrap());
        rules.write_buffer_connection = Some(WriteBufferConnection::Reading(format!(
            "mock://{}",
            state.id()
        )));
        if let WriteBufferConfig::Reading(conn) = WriteBufferConfig::new(server_id, &rules)
            .await
            .unwrap()
            .unwrap()
        {
            let conn = conn.lock().await;
            assert_eq!(conn.type_name(), "mock");
        } else {
            panic!("not a reading connection");
        }

        // will error when state is unknown
        drop(state);
        let err = WriteBufferConfig::new(server_id, &rules).await.unwrap_err();
        assert!(err.to_string().starts_with("Unknown mock ID:"));
    }
}
