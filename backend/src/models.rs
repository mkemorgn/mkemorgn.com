use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};

use crate::schema::trips;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Trip {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[diesel(table_name = trips)]
pub struct NewTrip<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TripPayload {
    pub title: String,
    pub body: String,
}
