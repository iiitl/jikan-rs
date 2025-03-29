use crate::common::wait_between_tests;
use jikan_rs::{JikanClient, JikanError};
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
async fn get_club_by_id() {
    let client = JikanClient::new();
    let result = client.get_club_by_id(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_club_members() {
    let client = JikanClient::new();
    let result = client.get_club_members(1, Some(1)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_club_staff() {
    let client = JikanClient::new();
    let result = client.get_club_staff(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_club_relations() {
    let client = JikanClient::new();
    let result = client.get_club_relations(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_club_search() {
    let client = JikanClient::new();
    let result = client
        .get_club_search(
            Some(1),
            Some(10),
            Some("anime".to_string()),
            None,
            None,
            None,
            None,
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_club_search_with_category() {
    let client = JikanClient::new();
    let result = client
        .get_club_search(
            Some(1),
            Some(5),
            None,
            Some("manga".to_string()),
            None,
            None,
            None,
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_club_search_with_order() {
    let client = JikanClient::new();
    let result = client
        .get_club_search(
            Some(1),
            Some(10),
            None,
            None,
            Some("members_count".to_string()),
            Some("desc".to_string()),
            None,
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_club_search_with_letter() {
    let client = JikanClient::new();
    let result = client
        .get_club_search(
            Some(1),
            Some(10),
            None,
            None,
            None,
            None,
            Some("A".to_string()),
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_nonexistent_club() {
    let client = JikanClient::new();
    let result = client.get_club_by_id(999999999).await;
    assert!(matches!(result, Err(JikanError::NotFound)));
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_nonexistent_club_members() {
    let client = JikanClient::new();
    let result = client.get_club_members(999999999, Some(1)).await;
    assert!(matches!(result, Err(JikanError::NotFound)));
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_nonexistent_club_staff() {
    let client = JikanClient::new();
    let result = client.get_club_staff(999999999).await;
    assert!(matches!(result, Err(JikanError::NotFound)));
    wait_between_tests().await;
}
