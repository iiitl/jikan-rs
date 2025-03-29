// misc.rs
use crate::{common::Images, users::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub mal_id: i32,
    pub url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub reactions: Option<ReviewReactions>,
    pub date: String,
    pub review: String,
    pub score: i32,
    pub tags: Vec<String>,
    pub is_spoiler: bool,
    pub is_preliminary: bool,
    pub episodes_watched: Option<i32>,
    pub user: ReviewUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewReactions {
    pub overall: i32,
    pub nice: i32,
    pub love_it: i32,
    pub funny: i32,
    pub confusing: i32,
    pub informative: i32,
    pub well_written: i32,
    pub creative: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalLink {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub relation: String,
    pub entry: Vec<RelatedEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedEntry {
    pub mal_id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationResponse {
    pub data: Vec<Relation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub entry: RecommendationEntry,
    pub votes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationEntry {
    pub mal_id: i32,
    pub url: String,
    pub images: Images,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub score: i32,
    pub votes: i32,
    pub percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Picture {
    pub jpg: Option<ImageFormat>,
    pub webp: Option<ImageFormat>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFormat {
    pub image_url: String,
    pub small_image_url: String,
    pub large_image_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ForumFilter {
    All,
    Episode,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumTopic {
    pub mal_id: i32,
    pub url: String,
    pub title: String,
    pub date: String,
    pub author_username: String,
    pub author_url: String,
    pub comments: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsItem {
    pub mal_id: i32,
    pub url: String,
    pub title: String,
    pub date: String,
    pub author_username: String,
    pub author_url: String,
    pub forum_url: String,
    pub images: Images,
    pub comments: i32,
    pub excerpt: String,
}
