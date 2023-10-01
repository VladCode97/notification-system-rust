use rocket::{get, post};
use rocket::serde::{json::Json};
use crate::model::message_model::Message;
use crate::database::message_collection::{view_message};
use crate::controllers::message_controller::message_manager;
#[get("/")]
pub fn index()  {
    view_message();
}

#[post("/", data = "<message>")]
pub fn create_message(message: Json<Message>) -> String {
    message_manager(Message { name: message.name.to_string(), create_at: message.name.to_string() });
    return "Message created successfully ".to_string() + &*message.create_at.to_string()
}