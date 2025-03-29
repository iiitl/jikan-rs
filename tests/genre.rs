use crate::common::wait_between_tests;
use jikan_rs::{JikanClient, JikanError};
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
async fn get_anime_genres() {
    let client = JikanClient::new();
    let result = client.get_anime_genres().await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_genres() {
    let client = JikanClient::new();
    let result = client.get_manga_genres().await;
    assert!(result.is_ok());
    wait_between_tests().await;
}
