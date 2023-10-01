use std::sync::{Mutex};
use crate::model::message_model::Message;

static MESSAGE_VECTOR: Mutex<Vec<Message>> = Mutex::new(Vec::new());

pub fn insert_message(message: Message) {
    MESSAGE_VECTOR.lock().unwrap().push(message);
}

pub fn view_message()  {
    for message in MESSAGE_VECTOR.lock().unwrap().iter() {
        dbg!(message);
    }
}