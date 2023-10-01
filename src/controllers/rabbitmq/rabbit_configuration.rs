use std::sync::Mutex;
use amiquip::{Connection, Exchange, Publish};
use crate::model::message_model::Message as MessageModel;

pub fn rabbit_configuration(message: MessageModel) {
    let mutex_rabbit_connection: Mutex<Connection> = Mutex::new(
        Connection::insecure_open("amqp://guest:guest@localhost:5672").expect("Error connection"));
    let channel = mutex_rabbit_connection.lock().unwrap()
        .open_channel(None).expect("Error getting chanel");
    let exchange = Exchange::direct(&channel);
    exchange.publish(Publish::new(message.name.as_bytes(), "messages"))
        .expect("Error publishing message");
    std::mem::drop(mutex_rabbit_connection.lock().unwrap());
}