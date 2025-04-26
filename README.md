# 🚀 AI Safety Incident API
you will enjoy reading this 😉😊

and if you don't want to read all this, just scroll a little down — you'll find a quick professional summary — short, crisp, no drama 😅.

yo! If you're reading this, trust me, I'm writing this not completely with AI 😅 :) please ignore small spelling mistakes.
This is an API project assignment given by a company. (I added the question below for reference.)
The task was simple: create an API.

#✨ Why I used Rust
I'm a cybersecurity explorer.
While facing different challenges, I came across Rust — found it super interesting, and decided to learn it...
4 months of painful struggle later 😭, I got just a little good at it — no time to master it fully.

Still, Rust is awesome:  
🚀 Super fast  
🛡️ Memory safe  
🔥 Very secure  
🖥️ Low CPU usage  
⚙️ Error handling like a boss  
💬 C++-like syntax, but oh boy... it's like hard mode unlocked 😂 — you have to think hard to code hard.


# 🎯 Quick Professional Summary  

🏢 Backend Intern Take-Home Assignment: HumanChain AI Safety Incident Log API  

Goal: Assess fundamental backend development skills — API design, request handling, data persistence.  
Context: HumanChain is a deep-tech AI startup working on AI Safety, trying to make the digital world safer, trustworthy, and human-centric.  
Assignment Build a simple backend API service to log and manage AI safety incidents.  

**quick note :- before diving into project you might face some bugs which imay not provided all solutions i also faced few silly bugs such as rocket failure , main.rs not sends request just changed the codes from redit and stackoverflow to see the most solution that worked for everyone home you face no bug while following my steps :)**

🖥️ My Setup  
OS: WSL2 with Kali Linux  
Language: Rust  
Framework: Rocket  
Database: SQLite  
ORM & Query Builder: Diesel  
Some Tools: chrono, serde, dotenv, git  

treestructure:-
During build  
![Screenshot 4](https://github.com/jaswanth6988/ai-safety-incidents/blob/main/assets/screenshot%20(4).png)
```
┌──(jash㉿DESKTOP-OIUGBAA)-[/mnt/d/KaliWsl/ai-safety-incidents]
└─$ tree -L 3 -I 'target|.git'
.
├── assets
│   ├── screenshot (1).png
│   ├── screenshot (2).png
│   ├── screenshot (3).png
│   ├── screenshot (4).png
│   ├── screenshot (5).png
│   ├── screenshot (6).png
│   └── screenshot (7).png
├── Cargo.lock
├── Cargo.toml
├── data
│   └── incidents.db
├── diesel.toml
├── migrations
│   └── 2025-04-26-191207_create_incidents
│       ├── down.sql
│       └── up.sql
├── README.md
├── Rocket.toml
└── src
    ├── db.rs
    ├── main.rs
    ├── models.rs
    └── schema.rs

6 directories, 19 files
```

⚡ Why These Choices?  

Choice	Reason  
Rust =>	Memory safety (no segmentfaults), blazing fast, amazing error handling.  
Rocket	=> Compile-time checks, fast request handling, secure & easy to use.  
SQLite	=> Lightweight, single-file DB — no server setup needed.  
Diesel	=> Safe and efficient ORM for Rust + SQLite.  
 

A secure, lightweight REST API for logging and managing AI safety incidents, built with Rust, Rocket, and Diesel (SQLite).

🌟 Features

Endpoint	Method	Description  
/incidents	GET	List all incidents  
/incidents	POST	Create a new incident  
/incidents/{id}	GET	Get details of a specific incident  
/incidents/{id}	DELETE	Delete an incident  


# 📦 Incident Model:

```
{
  "id": 1,
  "title": "Model Bias Detected",
  "description": "Gender bias found in hiring recommendations",
  "severity": "High",
  "reported_at": "2025-04-27T12:00:00Z"
}
```


# 🛠️ Setup

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org)  
[![Rocket](https://img.shields.io/badge/Rocket-0.5-red.svg)](https://rocket.rs)  
[![Diesel](https://img.shields.io/badge/Diesel-SQLite-green.svg)](https://diesel.rs) 

Prerequisites  
Rust 1.70+ →  ``` curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh ```  
SQLite3 → ``` sudo apt install sqlite3 libsqlite3-dev ```  
Diesel CLI → ```cargo install diesel_cli --no-default-features --features sqlite```  

# 📌 Create the Project  

1. Create a new Rust project
```
cargo new ai-safety-incidents
cd ai-safety-incidents
```


Open Cargo.toml and replace its contents with:
```
[package]
name = "ai-safety-incidents"
version = "0.1.0"
edition = "2021"

[dependencies]
rocket = { version = "0.5.0", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
diesel = { version = "2.0.0", features = ["sqlite"] }
dotenv = "0.15"
chrono = { version = "0.4", features = ["serde"] }
rocket_sync_db_pools = { version = "0.1.0", features = ["diesel_sqlite_pool"] }
```

Create a .env file
```
echo DATABASE_URL=sqlite://data/incidents.db > .env
```

Initialize the database
```
diesel setup
```

creat a migration 
```
diesel migration generate create_incidents
```

Edit migrations/YYYYMMDD_create_incidents/up.sql (just in case saying so it could be in date format)
```
CREATE TABLE incidents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    severity TEXT NOT NULL,
    reported_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```
Edit migrations/YYYYMMDD_create_incidents/down.sql (just in case saying so it could be in date format)
```
DROP TABLE incidents;
```

Run the migration
```
diesel migration run
```

Create src/schema.rs
```
diesel::table! {
    incidents (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        severity -> Text,
        reported_at -> Timestamp,
    }
}
```

on Main.rs
```
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
```
in same folder src/de.rs
```
use rocket_sync_db_pools::database;

#[database("sqlite_incidents")]
pub struct DbConn(diesel::SqliteConnection);
```

and lastly src/models.rs
```
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
```

then=>
if test ?

```
cargo check 
or 
cargo test ( for through check)

cargo build

cargo run

```
![Screenshot 1](https://github.com/jaswanth6988/ai-safety-incidents/blob/main/assets/screenshot%20(1).png)
![Screenshot 2](https://github.com/jaswanth6988/ai-safety-incidents/blob/main/assets/screenshot%20(2).png)

danm your server runing man get the job done now!


📡 API Usage
Create an Incident

creats an request to our server to register a new incident
```
curl -X POST -H "Content-Type: application/json" -d '{
  "title": "Data Leakage",
  "description": "Training data found in test set",
  "severity": "Medium"
}' http://localhost:8000/incidents
```
List All Incidents
```
curl http://localhost:8000/incidents
```
just ignore error in middle its just a error because iam not in folder so it cant locate file
![Screenshot 5](https://github.com/jaswanth6988/ai-safety-incidents/blob/main/assets/screenshot%20(5).png)

 POST (i just pasted what i used in my terminal danm just change you data you want)
```
 sqlite3 data/incidents.db "INSERT INTO incidents (title, description, severity, reported_at) VALUES
('Model bias detected', 'The model showed gender bias in hiring recommendations', 'High', datetime('now')),
('Data leakage', 'Training data was accidentally in test set', 'Medium', datetime('now'));"
or
 curl -X POST -H "Content-Type: application/json" -d '{
  "title": "Server Test",
  "description": "Testing from second terminal",
  "severity": "Medium"
}' http://localhost:8000/incidents
{"id":1,"title":"Server Test","description":"Testing from second terminal","severity":"Medium","reported_at":"2025-04-26T19:56:30"}
```

GET
```
curl http://localhost:8000/incidents
```
![Screenshot 6](https://github.com/jaswanth6988/ai-safety-incidents/blob/main/assets/screenshot%20(6).png)


DELETE
```
curl -X DELETE http://localhost:8000/incidents/1
```

ERROR(This Response i will try to add images)
![Screenshot 3](https://github.com/jaswanth6988/ai-safety-incidents/blob/main/assets/screenshot%20(3).png)
```
┌──(jash㉿DESKTOP-OIUGBAA)-[/mnt/d/KaliWsl/ai-safety-incidents]
└─$ curl -X DELETE http://localhost:8000/incidents/2
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="color-scheme" content="light dark">
    <title>404 Not Found</title>
</head>
<body align="center">
    <div role="main" align="center">
        <h1>404: Not Found</h1>
        <p>The requested resource could not be found.</p>
        <hr />
    </div>
    <div role="contentinfo" align="center">
        <small>Rocket</small>
    </div>
</body>
</html>
```

git pusing to github(just pasted my terminal for refrence and removed unwanted things)
```

211  git init

212  git status


213  cat > .gitignore << 'EOF'
# Rust
/target/
**/*.rs.bk

# Database
/data/
*.db
*.db-wal
*.db-shm

# Environment
.env

# IDE
.vscode/
.idea/


  215  ls

  216  git add Cargo.toml Cargo.lock Rocket.toml diesel.toml migrations/ src/ .gitignore

  217  git commit -m "Initial commit: AI Safety Incident API with Rocket and Diesel"

  218  git remote add origin https://github.com/jaswanth6988/ai-safety-incidents.git

  219  git remote add origin https://github.com/jaswanth6988/ai-safety-incidents.git

  220  git branch -M main

  221  git push -u origin main

  222  git status
```

# 👋 Thanks for Reading!

Trust me, Rust will punch you hard at first, but when you get it... it feels like controlling a beast 🚀.
Happy coding and don forget to hire me 😉!
