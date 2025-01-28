use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub last_visible_page: i32,
    pub has_next_page: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationPlus {
    pub last_visible_page: i32,
    pub has_next_page: bool,
    pub items: PaginationItems,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationItems {
    pub count: i32,
    pub total: i32,
    pub per_page: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Images {
    pub jpg: ImageSet,
    pub webp: Option<ImageSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSet {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>,
    pub large_image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}
