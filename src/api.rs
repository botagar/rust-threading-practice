use crate::system;
use rocket::State;
use rocket_contrib::json::Json;
use serde_json::Value;
use std::sync::{Arc,RwLock};

#[get("/")]
pub fn get_all(state: State<Arc<RwLock<system::State>>>) -> Json<Value> {
    return Json(json!(&*state.read().unwrap()));
}

pub fn start_api(state: Arc<RwLock<system::State>>) {
    rocket::ignite()
        .manage(state)
        .mount("/", routes![get_all]).launch();
}