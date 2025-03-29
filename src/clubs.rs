// clubs.rs
use crate::{
    common::{Images, Pagination},
    JikanClient, JikanError,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubResponse<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubVectorResponse<T> {
    pub data: Vec<T>,
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Club {
    pub mal_id: i32,
    pub url: String,
    pub images: Images,
    pub name: String,
    pub members: i32,
    pub category: String,
    pub created: String,
    pub access: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubMember {
    pub username: String,
    pub url: String,
    pub images: Images,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubStaff {
    pub url: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubRelationContent {
    pub mal_id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubRelations {
    pub anime: Vec<ClubRelationContent>,
    pub manga: Vec<ClubRelationContent>,
    pub characters: Vec<ClubRelationContent>,
}

impl JikanClient {
    pub async fn get_club_by_id(&self, id: i32) -> Result<ClubResponse<Club>, JikanError> {
        self.get(&format!("/clubs/{}", id)).await
    }

    pub async fn get_club_members(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<ClubVectorResponse<ClubMember>, JikanError> {
        let mut params = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/clubs/{}/members{}", id, query)).await
    }

    pub async fn get_club_staff(
        &self,
        id: i32,
    ) -> Result<ClubVectorResponse<ClubStaff>, JikanError> {
        self.get(&format!("/clubs/{}/staff", id)).await
    }

    pub async fn get_club_relations(
        &self,
        id: i32,
    ) -> Result<ClubResponse<ClubRelations>, JikanError> {
        self.get(&format!("/clubs/{}/relations", id)).await
    }

    pub async fn get_club_search(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
        q: Option<String>,
        category: Option<String>,
        order_by: Option<String>,
        sort: Option<String>,
        letter: Option<String>,
    ) -> Result<ClubVectorResponse<Club>, JikanError> {
        let mut params = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }

        if let Some(qr) = q {
            params.push(format!("q={}", qr));
        }

        if let Some(c) = category {
            params.push(format!("category={}", c));
        }

        if let Some(ob) = order_by {
            params.push(format!("order_by={}", ob));
        }

        if let Some(s) = sort {
            params.push(format!("sort={}", s));
        }

        if let Some(lt) = letter {
            params.push(format!("letter={}", lt));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/clubs{}", query)).await
    }
}
