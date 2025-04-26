use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::incidents)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Incident {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub severity: String,
    pub reported_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::incidents)]
pub struct NewIncident {
    pub title: String,
    pub description: String,
    pub severity: String,
}
