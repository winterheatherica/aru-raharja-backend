use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;
use crate::models::role::Role;

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(table_name = users)]
#[diesel(belongs_to(Role, foreign_key = role_id))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
    pub role_id: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
    pub role_id: i16,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub full_name: Option<String>,
    pub role_id: Option<i16>,
    pub updated_at: Option<DateTime<Utc>>,
}
