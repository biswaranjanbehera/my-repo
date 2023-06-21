use uuid::Uuid;
use rocket::post;
use rocket::response::status;
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::Queryable;
use diesel::pg::types::sql_types::Json;
use rocket::Rocket;
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;

#[macro_use] extern crate rocket;


//#[derive(Queryable, Identifiable, Serialize, Deserialize)]
// #[table_name = "raw_events"]
#[derive(Serialize, Deserialize)]
pub struct RawAppEvent{

    // #[serde(skip_serializing)]
    pub id: Uuid,

    // #[serde(skip_serializing)]
    pub identityId: Uuid,

    pub tenantId: String,

    //eventype
    pub eventType: EventType,

    // #[serde(skip_serializing)]
    pub sessionId: Uuid,

    pub applicationName: String,

    pub title: String,

    pub url: String,

    pub path: String,

    pub imageData: String,

    startTimeEpoch: i64,

    endTimeEpoch: i64,

    pub activeTime: i32,

    pub idleTime: i32,

    pub duration: i32,

    pub keystrokeCount: i32,

    pub mouseClicks: i32,

    //System status
    pub systemStatus: SystemStatus,

    // #[serde(skip_serializing)]
    pub imageUUID: Uuid,

    pub logStatus: bool,

    pub timeZone: String






    // pub created: std::time::SystemTime    
}

// impl RawAppEvent {
//     fn new() -> Self {
//         Self { 
//             id: Uuid::new_v4(),
//             sessionId: Uuid::new_v4()
//         }
//     }
// }

enum EventType{
    STANDARD, 
    APP_LOG, 
    SYSTEM, 
    PUNCH_IN, 
    PUNCH_OUT, 
    BREAK_START, 
    BREAK_END, 
    POWER_LOG, 
    AFK, 
    UNKNOWN,
    IDLE
}

enum SystemStatus{
    ACTIVE, 
    SUSPENDED, 
    LOCKED, 
    UNKNOWN
}


#[derive(Serialize, Deserialize)]
struct SimpleResponse {
    message: String,
}

#[post("/endpoint", format = "json", data = "<my_struct>")]
async fn my_endpoint(my_struct: Json<RawAppEvent>) -> Json<SimpleResponse> {
    let response = SimpleResponse {
        message: format!(
            "Hello, {}! You are {} years old.",
            my_struct.id, my_struct.url
        ),
    };
    Json(response)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/endpoint", routes![my_endpoint])
}

// fn main() {
//     rocket().launch();
// }