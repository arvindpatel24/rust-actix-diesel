use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name=crate::models::schema::events)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub location: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[diesel(table_name=crate::models::schema::events)]
pub struct NewEvent {
    pub name: String,
    pub description: String,
    pub location: String,
}
