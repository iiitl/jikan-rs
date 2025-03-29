use crate::common::wait_between_tests;
use jikan_rs::JikanClient;
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
async fn get_user_full() {
    let client = JikanClient::new();
    let result = client.get_user_full("InSaiyan__").await; // github.com/In-Saiyan
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_user() {
    let client = JikanClient::new();
    let result = client.get_user("InSaiyan__").await; //  github.com/In-Saiyan
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_users() {
    let client = JikanClient::new();
    let result = client.get_users().await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_user_by_id() {
    let client = JikanClient::new();
    let result = client.get_user_by_id(15847568).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_user_stats() {
    let client = JikanClient::new();
    let result = client.get_user_stats("InSaiyan__").await; //  github.com/In-Saiyan
    assert!(result.is_ok());
    wait_between_tests().await;
}
