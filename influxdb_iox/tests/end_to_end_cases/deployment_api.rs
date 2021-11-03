use influxdb_iox_client::{management::generated_types::DatabaseRules, write::WriteError};
use tonic::Code;

use crate::common::server_fixture::ServerFixture;

#[tokio::test]
async fn test_serving_readiness() {
    let server_fixture = ServerFixture::create_single_use().await;
    let mut deployment_client = server_fixture.deployment_client();
    let mut mgmt_client = server_fixture.management_client();
    let mut write_client = server_fixture.write_client();

    let name = "foo";
    let lp_data = "bar baz=1 10";

    deployment_client
        .update_server_id(42)
        .await
        .expect("set ID failed");
    server_fixture.wait_server_initialized().await;
    mgmt_client
        .create_database(DatabaseRules {
            name: name.to_string(),
            ..Default::default()
        })
        .await
        .expect("create database failed");

    assert!(deployment_client.get_serving_readiness().await.unwrap());

    deployment_client
        .set_serving_readiness(false)
        .await
        .unwrap();
    let err = write_client.write(name, lp_data).await.unwrap_err();
    assert!(
        matches!(&err, WriteError::ServerError(status) if status.code() == Code::Unavailable),
        "{}",
        &err
    );

    assert!(!deployment_client.get_serving_readiness().await.unwrap());

    deployment_client.set_serving_readiness(true).await.unwrap();
    assert!(deployment_client.get_serving_readiness().await.unwrap());
    write_client.write(name, lp_data).await.unwrap();
}

#[tokio::test]
async fn test_set_get_writer_id() {
    let server_fixture = ServerFixture::create_single_use().await;
    let mut client = server_fixture.deployment_client();

    const TEST_ID: u32 = 42;

    client
        .update_server_id(TEST_ID)
        .await
        .expect("set ID failed");

    let got = client.get_server_id().await.expect("get ID failed");

    assert_eq!(got.get(), TEST_ID);
}