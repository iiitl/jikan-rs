use crate::common::wait_between_tests;
use jikan_rs::JikanClient;
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
async fn get_anime_random() {
    let client = JikanClient::new();
    let result = client.get_random_anime().await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_random() {
    let client = JikanClient::new();
    let result = client.get_random_manga().await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_user_random() {
    let client = JikanClient::new();
    let result = client.get_random_user().await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_character_random() {
    let client = JikanClient::new();
    let result = client.get_random_character().await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_person_random() {
    let client = JikanClient::new();
    let result = client.get_random_person().await;
    assert!(result.is_ok());
    wait_between_tests().await;
}
