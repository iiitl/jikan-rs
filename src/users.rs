//user.rs
use crate::{
  common::Pagination,
  JikanClient, JikanError, anime::User,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse<T> {
  pub data: T,
  pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserVectorResponse<T>{
  pub data: Vec<T>,
  pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserById{
  pub url:String, 
  pub username:String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAnimeInfo{
  pub days_watched: f32,
  pub mean_score: f32,
  pub watching: i32,
  pub completed: i32,
  pub on_hold: i32,
  pub dropped: i32,
  pub plan_to_watch: i32,
  pub total_entries: i32,
  pub rewatched: i32,
  pub episodes_watched: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMangaInfo{
  pub days_read: f32,
  pub mean_score: f32,
  pub reading: i32,
  pub completed: i32,
  pub on_hold: i32,
  pub dropped: i32,
  pub plan_to_read: i32,
  pub total_entries: i32,
  pub reread: i32,
  pub chapters_read: i32,
  pub volumes_read: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats{
  pub anime: UserAnimeInfo,
  pub manga: UserMangaInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatsResponse{
  pub data: UserStats,
  pub pagination: Option<Pagination>,
}

impl JikanClient {
  pub async fn get_user_full(&self, username: &str) -> Result<UserResponse<User>, JikanError> {
    self.get(&format!("/users/{}/full", username)).await
  }

  pub async fn get_user(&self, username: &str) -> Result<UserResponse<User>, JikanError> {
    self.get(&format!("/users/{}", username)).await
  }

  pub async fn get_users(&self) -> Result<UserVectorResponse<User>, JikanError> {
    self.get(&format!("/users/")).await
  }

  pub async fn get_user_by_id(&self, id: i32) -> Result<UserResponse<UserById>, JikanError> { //Maybe handle this better?
    self.get(&format!("/users/userbyid/{}", id)).await
  }

  pub async fn get_user_stats(&self, username: &str) -> Result<UserStatsResponse, JikanError> {
    self.get(&format!("/users/{}/statistics", username)).await
  } 
}

// to continue