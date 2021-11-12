use super::scenario::{create_readable_database, rand_name};
use crate::{
    common::server_fixture::{ServerFixture, ServerType},
    end_to_end_cases::scenario::{
        fixture_broken_catalog, fixture_replay_broken, list_chunks, wait_for_exact_chunk_states,
        DatabaseBuilder,
    },
};
use assert_cmd::Command;
use data_types::chunk_metadata::ChunkStorage;
use generated_types::{
    google::longrunning::IoxOperation,
    influxdata::iox::management::v1::{operation_metadata::Job, WipePreservedCatalog},
};
use predicates::prelude::*;
use std::time::Duration;
use tempfile::TempDir;
use test_helpers::make_temp_file;
use uuid::Uuid;

#[tokio::test]
async fn test_server_id() {
    let server_fixture = ServerFixture::create_single_use(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("server")
        .arg("set")
        .arg("32")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("Ok"));

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("server")
        .arg("get")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("32"));

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("server")
        .arg("set")
        .arg("42")
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains("id already set"));
}

#[tokio::test]
async fn test_create_database() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();
    let db = &db_name;

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("get")
        .arg(db)
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Database not found"));

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("create")
        .arg(db)
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("Created database {}", db)));

    // Listing the databases includes the newly created database
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("list")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains(db));

    // Retrieving the database includes the name and a mutable buffer configuration
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("get")
        .arg(db)
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(
            predicate::str::contains(db)
                .and(predicate::str::contains(format!(r#""name": "{}"#, db)))
                // validate the defaults have been set reasonably
                .and(predicate::str::contains("%Y-%m-%d %H:00:00"))
                .and(predicate::str::contains(r#""bufferSizeHard": "104857600""#))
                .and(predicate::str::contains("lifecycleRules")),
        );
}

#[tokio::test]
async fn test_create_database_size() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();
    let db = &db_name;

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("create")
        .arg(db)
        .arg("--buffer-size-hard")
        .arg("1000")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created"));

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("get")
        .arg(db)
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(
            predicate::str::contains(r#""bufferSizeHard": "1000""#)
                .and(predicate::str::contains("lifecycleRules")),
        );
}

#[tokio::test]
async fn test_create_database_immutable() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();
    let db = &db_name;

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("create")
        .arg(db)
        .arg("--immutable")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created"));

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("get")
        .arg(db)
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        // Should not have a mutable buffer
        .stdout(predicate::str::contains(r#""immutable": true"#));
}

#[tokio::test]
async fn delete_restore_database() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();
    let db = &db_name;

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("create")
        .arg(db)
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created"));

    // Listing the databases includes the newly created database
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("list")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains(db));

    // Listing detailed database info includes the active database
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("list")
        .arg("--detailed")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains(db));

    // Delete the database, returns the UUID
    let stdout = String::from_utf8(
        Command::cargo_bin("influxdb_iox")
            .unwrap()
            .arg("database")
            .arg("delete")
            .arg(db)
            .arg("--host")
            .arg(addr)
            .assert()
            .success()
            .stdout(predicate::str::contains(format!("Deleted database {}", db)))
            .get_output()
            .stdout
            .clone(),
    )
    .unwrap();
    let db_uuid = stdout.lines().last().unwrap().trim();

    // Listing the databases does not include the deleted database
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("list")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains(db).not());

    // Deleting the database again is an error
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("delete")
        .arg(db)
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Error deleting database: Database not found",
        ));

    // Creating a new database with the same name works
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("create")
        .arg(db)
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created"));

    // The newly-created database will be in the active list
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("list")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains(db));

    // Restoring the 1st database is an error because the new, currently active database has the
    // same name
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("restore")
        .arg(db_uuid)
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "A database with the name `{}` already exists",
            db
        )));

    // Delete the 2nd database
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("delete")
        .arg(db)
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("Deleted database {}", db)));

    // The 2nd database should no longer be in the active list
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("list")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains(db).not());

    // Restore the 1st database
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("restore")
        .arg(db_uuid)
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "Restored database {}",
            db_uuid
        )));

    // The 1st database is back in the active list
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("list")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains(db));

    // Restoring again is an error
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("restore")
        .arg(db_uuid)
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "The database with UUID `{}` named `{}` is already active",
            db_uuid, db
        )));

    // Restoring a database with a valid but unknown UUID is an error
    let unknown_uuid = Uuid::new_v4();
    dbg!(unknown_uuid);
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("restore")
        .arg(unknown_uuid.to_string())
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "Could not find a database with UUID `{}`",
            unknown_uuid
        )));
}

#[tokio::test]
async fn disown_database() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();
    let db = &db_name;

    // Create a database on one server
    let stdout = String::from_utf8(
        Command::cargo_bin("influxdb_iox")
            .unwrap()
            .arg("database")
            .arg("create")
            .arg(db)
            .arg("--host")
            .arg(addr)
            .assert()
            .success()
            .stdout(predicate::str::contains("Created"))
            .get_output()
            .stdout
            .clone(),
    )
    .unwrap();
    let created_uuid = stdout.lines().last().unwrap().trim();

    // Disown database returns the UUID
    let stdout = String::from_utf8(
        Command::cargo_bin("influxdb_iox")
            .unwrap()
            .arg("database")
            .arg("disown")
            .arg(db)
            .arg("--host")
            .arg(addr)
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Disowned database {}",
                db
            )))
            .get_output()
            .stdout
            .clone(),
    )
    .unwrap();
    let deleted_uuid = stdout.lines().last().unwrap().trim();
    assert_eq!(created_uuid, deleted_uuid);

    // Disowned database is no longer in this server's database list
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("list")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains(db).not());

    // Disowning the same database again is an error
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("disown")
        .arg(db)
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "Could not find database {}",
            db
        )));

    // Create another database
    let stdout = String::from_utf8(
        Command::cargo_bin("influxdb_iox")
            .unwrap()
            .arg("database")
            .arg("create")
            .arg(db)
            .arg("--host")
            .arg(addr)
            .assert()
            .success()
            .stdout(predicate::str::contains("Created"))
            .get_output()
            .stdout
            .clone(),
    )
    .unwrap();
    let created_uuid = stdout.lines().last().unwrap().trim();

    // If an optional UUID is specified, don't disown the database if the UUID doesn't match
    let incorrect_uuid = Uuid::new_v4();
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("disown")
        .arg(db)
        .arg("--uuid")
        .arg(incorrect_uuid.to_string())
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "Could not disown {}: the UUID specified ({}) does not match the current UUID ({})",
            db, incorrect_uuid, created_uuid,
        )));

    // Error if the UUID specified is not in a valid UUID format
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("disown")
        .arg(db)
        .arg("--uuid")
        .arg("foo")
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Invalid value for '--uuid <uuid>'",
        ));

    // If an optional UUID is specified, disown the database if the UUID does match
    let stdout = String::from_utf8(
        Command::cargo_bin("influxdb_iox")
            .unwrap()
            .arg("database")
            .arg("disown")
            .arg(db)
            .arg("--uuid")
            .arg(created_uuid)
            .arg("--host")
            .arg(addr)
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Disowned database {}",
                db
            )))
            .get_output()
            .stdout
            .clone(),
    )
    .unwrap();
    let deleted_uuid = stdout.lines().last().unwrap().trim();
    assert_eq!(created_uuid, deleted_uuid);
}

#[tokio::test]
async fn adopt_database() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();
    let db = &db_name;

    // Create a database on one server
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("create")
        .arg(db)
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created"));

    // Disown database returns the UUID
    let stdout = String::from_utf8(
        Command::cargo_bin("influxdb_iox")
            .unwrap()
            .arg("database")
            .arg("disown")
            .arg(db)
            .arg("--host")
            .arg(addr)
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Disowned database {}",
                db
            )))
            .get_output()
            .stdout
            .clone(),
    )
    .unwrap();
    let deleted_uuid = stdout.lines().last().unwrap().trim();

    // Create another database with the same name
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("create")
        .arg(db)
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("Created"));

    // Disown the other database too
    let stdout = String::from_utf8(
        Command::cargo_bin("influxdb_iox")
            .unwrap()
            .arg("database")
            .arg("disown")
            .arg(db)
            .arg("--host")
            .arg(addr)
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Disowned database {}",
                db
            )))
            .get_output()
            .stdout
            .clone(),
    )
    .unwrap();
    let second_deleted_uuid = stdout.lines().last().unwrap().trim();

    // Adopt using the first UUID
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("adopt")
        .arg(deleted_uuid)
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("Adopted database {}", db)));

    // Adopted database is now in this server's database list
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("list")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains(db));

    // Adopting again is an error
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("adopt")
        .arg(deleted_uuid)
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "The database with UUID `{}` is already owned by this server",
            deleted_uuid
        )));

    // Error if the UUID specified is not in a valid UUID format
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("adopt")
        .arg("foo")
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid value for '<uuid>'"));

    // Adopting a valid but unknown UUID is an error
    let unknown_uuid = Uuid::new_v4();
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("adopt")
        .arg(unknown_uuid.to_string())
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "Could not find a database with UUID `{}`",
            unknown_uuid
        )));

    // Adopting the second db that has the same name as the existing database is an error
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("adopt")
        .arg(second_deleted_uuid)
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "A database with the name `{}` already exists",
            db
        )));
}

#[tokio::test]
async fn test_get_chunks() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();

    create_readable_database(&db_name, server_fixture.grpc_channel()).await;

    let lp_data = vec![
        "cpu,region=west user=23.2 100",
        "cpu,region=west user=21.0 150",
    ];

    load_lp(addr, &db_name, lp_data);

    let predicate = predicate::str::contains(r#""partitionKey": "cpu","#)
        .and(predicate::str::contains(
            r#""storage": "CHUNK_STORAGE_OPEN_MUTABLE_BUFFER","#,
        ))
        .and(predicate::str::contains(r#""memoryBytes": "1048""#))
        // Check for a non empty timestamp such as
        // "time_of_first_write": "2021-03-30T17:11:10.723866Z",
        .and(predicate::str::contains(r#""timeOfFirstWrite": "20"#));

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("chunk")
        .arg("list")
        .arg(&db_name)
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate);
}

#[tokio::test]
async fn test_list_chunks_error() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();

    // note don't make the database, expect error

    // list the chunks
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("chunk")
        .arg("list")
        .arg(&db_name)
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("Some requested entity was not found: Resource database")
                .and(predicate::str::contains(&db_name)),
        );
}

#[tokio::test]
async fn test_remotes() {
    let server_fixture = ServerFixture::create_single_use(ServerType::Database).await;
    let addr = server_fixture.grpc_base();

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("server")
        .arg("set")
        .arg("32")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("Ok"));

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("server")
        .arg("remote")
        .arg("list")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("no remotes configured"));

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("server")
        .arg("remote")
        .arg("set")
        .arg("1")
        .arg("http://1.2.3.4:1234")
        .arg("--host")
        .arg(addr)
        .assert()
        .success();

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("server")
        .arg("remote")
        .arg("list")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("http://1.2.3.4:1234"));

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("server")
        .arg("remote")
        .arg("remove")
        .arg("1")
        .arg("--host")
        .arg(addr)
        .assert()
        .success();

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("server")
        .arg("remote")
        .arg("list")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("no remotes configured"));
}

#[tokio::test]
async fn test_list_partitions() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();

    create_readable_database(&db_name, server_fixture.grpc_channel()).await;

    let lp_data = vec![
        "cpu,region=west user=23.2 100",
        "mem,region=west free=100000 150",
    ];
    load_lp(addr, &db_name, lp_data);

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("partition")
        .arg("list")
        .arg(&db_name)
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("cpu").and(predicate::str::contains("mem")));
}

#[tokio::test]
async fn test_list_partitions_error() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("partition")
        .arg("list")
        .arg("non_existent_database")
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Database not found"));
}

#[tokio::test]
async fn test_get_partition() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();

    create_readable_database(&db_name, server_fixture.grpc_channel()).await;

    let lp_data = vec![
        "cpu,region=west user=23.2 100",
        "mem,region=west free=100000 150",
    ];
    load_lp(addr, &db_name, lp_data);

    let expected = r#""key": "cpu""#;

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("partition")
        .arg("get")
        .arg(&db_name)
        .arg("cpu")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));
}

#[tokio::test]
async fn test_get_partition_error() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("partition")
        .arg("get")
        .arg("cpu")
        .arg("non_existent_database")
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Database not found"));
}

#[tokio::test]
async fn test_list_partition_chunks() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();

    create_readable_database(&db_name, server_fixture.grpc_channel()).await;

    let lp_data = vec![
        "cpu,region=west user=23.2 100",
        "cpu2,region=west user=21.0 150",
    ];

    load_lp(addr, &db_name, lp_data);

    let partition_key = "cpu";
    // should not contain anything related to cpu2 partition
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("partition")
        .arg("list-chunks")
        .arg(&db_name)
        .arg(&partition_key)
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(
            predicate::str::contains(r#""partitionKey": "cpu""#)
                .and(predicate::str::contains(r#""tableName": "cpu""#))
                .and(predicate::str::contains(r#""order": 1"#))
                .and(predicate::str::contains(
                    r#""storage": "CHUNK_STORAGE_OPEN_MUTABLE_BUFFER""#,
                ))
                .and(predicate::str::contains("cpu2").not()),
        );
}

#[tokio::test]
async fn test_list_partition_chunks_error() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();

    // note don't make the database, expect error

    // list the chunks
    let partition_key = "cpu";
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("partition")
        .arg("list-chunks")
        .arg(&db_name)
        .arg(&partition_key)
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("Some requested entity was not found: Resource database")
                .and(predicate::str::contains(&db_name)),
        );
}

#[tokio::test]
async fn test_new_partition_chunk() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();

    create_readable_database(&db_name, server_fixture.grpc_channel()).await;

    let lp_data = vec!["cpu,region=west user=23.2 100"];
    load_lp(addr, &db_name, lp_data);

    let expected = "Ok";
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("partition")
        .arg("new-chunk")
        .arg(&db_name)
        .arg("cpu")
        .arg("cpu")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains(expected));

    wait_for_exact_chunk_states(
        &server_fixture,
        &db_name,
        vec![ChunkStorage::ReadBuffer],
        std::time::Duration::from_secs(5),
    )
    .await;

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("chunk")
        .arg("list")
        .arg(&db_name)
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("CHUNK_STORAGE_READ_BUFFER"));
}

#[tokio::test]
async fn test_new_partition_chunk_error() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("partition")
        .arg("new-chunk")
        .arg("non_existent_database")
        .arg("non_existent_partition")
        .arg("non_existent_table")
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Resource database/non_existent_database not found",
        ));
}

#[tokio::test]
async fn test_close_partition_chunk() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();

    create_readable_database(&db_name, server_fixture.grpc_channel()).await;

    let lp_data = vec!["cpu,region=west user=23.2 100"];
    load_lp(addr, &db_name, lp_data);

    let chunks = list_chunks(&server_fixture, &db_name).await;
    let chunk_id = chunks[0].id;

    let iox_operation: IoxOperation = serde_json::from_slice(
        &Command::cargo_bin("influxdb_iox")
            .unwrap()
            .arg("database")
            .arg("partition")
            .arg("close-chunk")
            .arg(&db_name)
            .arg("cpu")
            .arg("cpu")
            .arg(chunk_id.get().to_string())
            .arg("--host")
            .arg(addr)
            .assert()
            .success()
            .get_output()
            .stdout,
    )
    .expect("Expected JSON output");

    match iox_operation.metadata.job {
        Some(Job::CompactChunks(job)) => {
            assert_eq!(job.chunks.len(), 1);
            assert_eq!(&job.db_name, &db_name);
            assert_eq!(job.partition_key.as_str(), "cpu");
            assert_eq!(job.table_name.as_str(), "cpu");
        }
        job => panic!("unexpected job returned {:#?}", job),
    }
}

#[tokio::test]
async fn test_close_partition_chunk_error() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("partition")
        .arg("close-chunk")
        .arg("non_existent_database")
        .arg("non_existent_partition")
        .arg("non_existent_table")
        .arg("00000000-0000-0000-0000-000000000000")
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Database not found"));
}

#[tokio::test]
async fn test_wipe_persisted_catalog() {
    let db_name = rand_name();
    let server_fixture = fixture_broken_catalog(&db_name).await;
    let addr = server_fixture.grpc_base();

    let stdout: IoxOperation = serde_json::from_slice(
        &Command::cargo_bin("influxdb_iox")
            .unwrap()
            .arg("database")
            .arg("recover")
            .arg("wipe")
            .arg(&db_name)
            .arg("--host")
            .arg(addr)
            .arg("--force")
            .assert()
            .success()
            .get_output()
            .stdout,
    )
    .expect("Expected JSON output");

    let expected_job = Job::WipePreservedCatalog(WipePreservedCatalog { db_name });

    assert_eq!(
        Some(expected_job),
        stdout.metadata.job,
        "operation was {:#?}",
        stdout
    );
}

#[tokio::test]
async fn test_wipe_persisted_catalog_error_force() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("recover")
        .arg("wipe")
        .arg(&db_name)
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Need to pass `--force`"));
}

#[tokio::test]
async fn test_wipe_persisted_catalog_error_db_exists() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();

    create_readable_database(&db_name, server_fixture.grpc_channel()).await;

    let expected_err = format!("Failed precondition: database ({}) in invalid state (Initialized) for transition (WipePreservedCatalog)", db_name);

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("recover")
        .arg("wipe")
        .arg(&db_name)
        .arg("--host")
        .arg(addr)
        .arg("--force")
        .assert()
        .failure()
        .stderr(predicate::str::contains(&expected_err));
}

#[tokio::test]
async fn test_skip_replay() {
    let write_buffer_dir = TempDir::new().unwrap();
    let db_name = rand_name();
    let server_fixture = fixture_replay_broken(&db_name, write_buffer_dir.path()).await;
    let addr = server_fixture.grpc_base();

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("recover")
        .arg("skip-replay")
        .arg(&db_name)
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("Ok"));
}

#[tokio::test]
async fn test_skip_replay_error_db_exists() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();

    create_readable_database(&db_name, server_fixture.grpc_channel()).await;

    let expected_err = format!("Failed precondition: database ({}) in invalid state (Initialized) for transition (SkipReplay)", db_name);

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("recover")
        .arg("skip-replay")
        .arg(&db_name)
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains(&expected_err));
}

/// Loads the specified lines into the named database
fn load_lp(addr: &str, db_name: &str, lp_data: Vec<&str>) {
    let lp_data_file = make_temp_file(lp_data.join("\n"));

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("write")
        .arg(&db_name)
        .arg(lp_data_file.as_ref())
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("Lines OK"));
}

#[tokio::test]
async fn test_unload_partition_chunk() {
    let fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = fixture.grpc_base();
    let db_name = rand_name();

    DatabaseBuilder::new(db_name.clone())
        .persist(true)
        .persist_age_threshold_seconds(1)
        .late_arrive_window_seconds(1)
        .build(fixture.grpc_channel())
        .await;

    let lp_data = vec!["cpu,region=west user=23.2 10"];
    load_lp(addr, &db_name, lp_data);

    let mut chunks = wait_for_exact_chunk_states(
        &fixture,
        &db_name,
        vec![ChunkStorage::ReadBufferAndObjectStore],
        std::time::Duration::from_secs(5),
    )
    .await;
    let chunk = chunks.pop().unwrap();

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("partition")
        .arg("unload-chunk")
        .arg(&db_name)
        .arg("cpu")
        .arg("cpu")
        .arg(chunk.id.get().to_string())
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("Ok"));
}

#[tokio::test]
async fn test_unload_partition_chunk_error() {
    let server_fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = server_fixture.grpc_base();
    let db_name = rand_name();

    create_readable_database(&db_name, server_fixture.grpc_channel()).await;

    let lp_data = vec!["cpu,region=west user=23.2 100"];
    load_lp(addr, &db_name, lp_data);

    let chunks = list_chunks(&server_fixture, &db_name).await;
    let chunk_id = chunks[0].id;

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("partition")
        .arg("unload-chunk")
        .arg(&db_name)
        .arg("cpu")
        .arg("cpu")
        .arg(chunk_id.get().to_string())
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains("wrong chunk lifecycle"));
}

#[tokio::test]
async fn test_drop_partition() {
    let fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = fixture.grpc_base();
    let db_name = rand_name();

    DatabaseBuilder::new(db_name.clone())
        .persist(true)
        .persist_age_threshold_seconds(1)
        .late_arrive_window_seconds(1)
        .build(fixture.grpc_channel())
        .await;

    let lp_data = vec!["cpu,region=west user=23.2 10"];
    load_lp(addr, &db_name, lp_data);

    wait_for_exact_chunk_states(
        &fixture,
        &db_name,
        vec![ChunkStorage::ReadBufferAndObjectStore],
        std::time::Duration::from_secs(5),
    )
    .await;

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("partition")
        .arg("drop")
        .arg(&db_name)
        .arg("cpu")
        .arg("cpu")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("Ok"));
}

#[tokio::test]
async fn test_drop_partition_error() {
    let fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = fixture.grpc_base();
    let db_name = rand_name();

    DatabaseBuilder::new(db_name.clone())
        .persist(true)
        .persist_age_threshold_seconds(1_000)
        .late_arrive_window_seconds(1_000)
        .build(fixture.grpc_channel())
        .await;

    let lp_data = vec!["cpu,region=west user=23.2 10"];
    load_lp(addr, &db_name, lp_data);

    wait_for_exact_chunk_states(
        &fixture,
        &db_name,
        vec![ChunkStorage::OpenMutableBuffer],
        std::time::Duration::from_secs(5),
    )
    .await;

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("partition")
        .arg("drop")
        .arg(&db_name)
        .arg("cpu")
        .arg("cpu")
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Cannot drop unpersisted chunk"));
}

#[tokio::test]
async fn test_persist_partition() {
    let fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = fixture.grpc_base();
    let db_name = rand_name();

    DatabaseBuilder::new(db_name.clone())
        .persist(true)
        .persist_age_threshold_seconds(1_000)
        .late_arrive_window_seconds(1)
        .build(fixture.grpc_channel())
        .await;

    let lp_data = vec!["cpu,region=west user=23.2 10"];
    load_lp(addr, &db_name, lp_data);

    wait_for_exact_chunk_states(
        &fixture,
        &db_name,
        vec![ChunkStorage::OpenMutableBuffer],
        std::time::Duration::from_secs(5),
    )
    .await;

    tokio::time::sleep(Duration::from_millis(1500)).await;

    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("partition")
        .arg("persist")
        .arg(&db_name)
        .arg("cpu")
        .arg("cpu")
        .arg("--host")
        .arg(addr)
        .assert()
        .success()
        .stdout(predicate::str::contains("Ok"));
}

#[tokio::test]
async fn test_persist_partition_error() {
    let fixture = ServerFixture::create_shared(ServerType::Database).await;
    let addr = fixture.grpc_base();
    let db_name = rand_name();

    DatabaseBuilder::new(db_name.clone())
        .persist(true)
        .persist_age_threshold_seconds(1_000)
        .late_arrive_window_seconds(1_000)
        .build(fixture.grpc_channel())
        .await;

    let lp_data = vec!["cpu,region=west user=23.2 10"];
    load_lp(addr, &db_name, lp_data);

    wait_for_exact_chunk_states(
        &fixture,
        &db_name,
        vec![ChunkStorage::OpenMutableBuffer],
        std::time::Duration::from_secs(5),
    )
    .await;

    // there is no old data (late arrival window is 1000s) that can be persisted
    Command::cargo_bin("influxdb_iox")
        .unwrap()
        .arg("database")
        .arg("partition")
        .arg("persist")
        .arg(&db_name)
        .arg("cpu")
        .arg("cpu")
        .arg("--host")
        .arg(addr)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error persisting partition:").and(
            predicate::str::contains(
                "Cannot persist partition because it cannot be flushed at the moment",
            ),
        ));
}
