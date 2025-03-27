use crate::common::wait_between_tests;
use jikan_rs::{JikanClient, JikanError};
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
async fn get_manga() {
    let client = JikanClient::new();
    let result = client.get_manga(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_full() {
    let client = JikanClient::new();
    let result = client.get_manga_full(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_characters() {
    let client = JikanClient::new();
    let result = client.get_manga_characters(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_news() {
    let client = JikanClient::new();
    let result = client.get_manga_news(1, Some(1)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_forum() {
    let client = JikanClient::new();
    let result = client.get_manga_forum(1, None).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_pictures() {
    let client = JikanClient::new();
    let result = client.get_manga_pictures(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_statistics() {
    let client = JikanClient::new();
    let result = client.get_manga_statistics(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_moreinfo() {
    let client = JikanClient::new();
    let result = client.get_manga_moreinfo(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_recommendations() {
    let client = JikanClient::new();
    let result = client.get_manga_recommendations(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_userupdates() {
    let client = JikanClient::new();
    let result = client.get_manga_userupdates(1, Some(1)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_reviews() {
    let client = JikanClient::new();
    let result = client.get_manga_reviews(1, Some(1), None, None).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_relations() {
    let client = JikanClient::new();
    let result = client.get_manga_relations(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_manga_external() {
    let client = JikanClient::new();
    let result = client.get_manga_external(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_nonexistent_manga() {
    let client = JikanClient::new();
    let result = client.get_manga(999999999).await;
    assert!(matches!(result, Err(JikanError::NotFound)));
    wait_between_tests().await;
}
