use crate::common::wait_between_tests;
use jikan_rs::JikanClient;
use serial_test::serial;
mod common;
use jikan_rs::character::*;

#[tokio::test]
#[serial]
pub async fn get_character_by_id() {
    let client = JikanClient::new();
    let result = client.get_character_by_id(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_character_full_by_id() {
    let client = JikanClient::new();
    let result = client.get_character_full_by_id(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_character_anime() {
    let client = JikanClient::new();
    let result = client.get_character_anime(1).await;
    println!("{:?}", result);
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_character_manga() {
    let client = JikanClient::new();
    let result = client.get_character_manga(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_character_voices() {
    let client = JikanClient::new();
    let result = client.get_character_voices(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_characters() {
    let client = JikanClient::new();
    let result = client.get_characters().await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_character_pictures() {
    let client = JikanClient::new();
    let result = client.get_character_pictures(1).await;
    println!("{:?}", result);
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
pub async fn get_character_search() {
    let client = JikanClient::new();
    let result = client
        .get_character_search(
            None,
            Some(1),
            Some(String::from("Naruto")),
            Some(OrderBy::Favorites),
            Some(Sort::Asc),
        )
        .await;
    println!("{:?}", result);
    assert!(result.is_ok());
    wait_between_tests().await;
}
