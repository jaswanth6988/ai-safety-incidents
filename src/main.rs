#[macro_use] extern crate rocket;
#[allow(unused_imports)] extern crate diesel;  // Changed this line

mod models;
mod schema;
mod db;

use rocket::serde::json::Json;
use rocket::http::Status;
use diesel::prelude::*;
use models::{Incident, NewIncident};
use db::DbConn;
use schema::incidents::dsl::*;

#[get("/incidents")]
async fn get_incidents(conn: DbConn) -> Result<Json<Vec<Incident>>, Status> {
    conn.run(|c| {
        incidents
            .select(Incident::as_select())
            .load(c)
    })
    .await
    .map(Json)
    .map_err(|_| Status::InternalServerError)
}

#[get("/incidents/<incident_id>")]
async fn get_incident(conn: DbConn, incident_id: i32) -> Result<Json<Incident>, Status> {
    conn.run(move |c| {
        incidents
            .filter(id.eq(incident_id))
            .select(Incident::as_select())
            .first(c)
    })
    .await
    .map(Json)
    .map_err(|e| match e {
        diesel::result::Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    })
}

#[post("/incidents", data = "<new_incident>")]
async fn create_incident(
    conn: DbConn, 
    new_incident: Json<NewIncident>
) -> Result<Json<Incident>, Status> {
    // Validate severity
    let valid_severities = ["Low", "Medium", "High"];
    if !valid_severities.contains(&new_incident.severity.as_str()) {
        return Err(Status::BadRequest);
    }

    conn.run(move |c| {
        // First insert the record
        diesel::insert_into(incidents)
            .values(&*new_incident)
            .execute(c)?;
            
        // Then get the last inserted record
        incidents
            .order(id.desc())
            .select(Incident::as_select())
            .first(c)
    })
    .await
    .map(Json)
    .map_err(|_| Status::InternalServerError)
}

#[delete("/incidents/<incident_id>")]
async fn delete_incident(conn: DbConn, incident_id: i32) -> Status {
    let result = conn.run(move |c| {
        diesel::delete(incidents.filter(id.eq(incident_id)))
            .execute(c)
    }).await;

    match result {
        Ok(1) => Status::NoContent,
        Ok(0) => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![
            get_incidents,
            get_incident,
            create_incident,
            delete_incident
        ])
}
