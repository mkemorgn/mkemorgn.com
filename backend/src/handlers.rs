use super::DbPool;

use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;

use crate::models::{NewTrip, Trip, TripPayload};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/trips")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let trips = web::block(move || {
        let mut conn = pool.get()?;
        find_all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(trips))
}

#[post("/trips")]
async fn create(
    pool: web::Data<DbPool>,
    payload: web::Json<TripPayload>,
) -> Result<HttpResponse, Error> {
    let trip = web::block(move || {
        let mut conn = pool.get()?;
        add_a_trip(&payload.title, &payload.body, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(trip))
}

#[get("/trips/{id}")]
async fn show(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let trip = web::block(move || {
        let mut conn = pool.get()?;
        find_by_id(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(trip))
}

#[put("/trips/{id}")]
async fn update(
    id: web::Path<i32>,
    payload: web::Json<TripPayload>,
    pool: web::Data<DbPool>,
    ) -> Result<HttpResponse, Error> {
    let trip = web::block(move || {
        let mut conn = pool.get()?;
        update_trip_title(id.into_inner(), payload.title.clone(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(trip))
}

#[delete("/trips/{id}")]
async fn destroy(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let mut conn = pool.get()?;
        delete_trip(id.into_inner(), &mut conn)
    })
    .await?
    .map(|trip| HttpResponse::Ok().json(trip))
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(result)
}

fn add_a_trip(_title: &str, _body: &str, conn: &mut PgConnection) -> Result<Trip, DbError> {
    use crate::schema::trips::dsl::*;

    let new_trip = NewTrip {
        title: _title,
        body: _body,
    };

    let res = diesel::insert_into(trips)
        .values(&new_trip)
        .get_result(conn)?;
    Ok(res)
}

fn find_all(conn: &mut PgConnection) -> Result<Vec<Trip>, DbError> {
    use crate::schema::trips::dsl::*;

    let items = trips.load::<Trip>(conn)?;
    Ok(items)
}

fn find_by_id(trip_id: i32, conn: &mut PgConnection) -> Result<Option<Trip>, DbError> {
    use crate::schema::trips::dsl::*;

    let trip = trips
        .filter(id.eq(trip_id))
        .first::<Trip>(conn)
        .optional()?;
    Ok(trip)
}

fn update_trip_title(trip_id: i32, _title: String, conn: &mut PgConnection) -> Result<Trip, DbError> {
    use crate::schema::trips::dsl::*;

    let trip = diesel::update(trips.find(trip_id))
        .set(title.eq(_title))
        .get_result::<Trip>(conn)?;

    Ok(trip)
}


fn delete_trip(trip_id: i32, conn: &mut PgConnection) -> Result<usize, DbError> {
    use crate::schema::trips::dsl::*;

    let count = diesel::delete(trips.find(trip_id)).execute(conn)?;
    Ok(count)
}
