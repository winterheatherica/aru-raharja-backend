use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::category;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = category)]
pub struct Category {
    pub id: i16,
    pub name: String,
    pub alias: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = category)]
pub struct NewCategory {
    pub name: String,
    pub alias: String,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = category)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub alias: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}
