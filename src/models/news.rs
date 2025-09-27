use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::news;
use crate::models::user::User;
use crate::models::category::Category;

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(table_name = news)]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(belongs_to(Category, foreign_key = category_id))]
pub struct News {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub author_id: Uuid,
    pub category_id: i16,
    pub thumbnail: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = news)]
pub struct NewNews {
    pub title: String,
    pub description: Option<String>,
    pub author_id: Uuid,
    pub category_id: i16,
    pub thumbnail: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = news)]
pub struct UpdateNews {
    pub title: Option<String>,
    pub description: Option<String>,
    pub author_id: Option<Uuid>,
    pub category_id: Option<i16>,
    pub thumbnail: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}
