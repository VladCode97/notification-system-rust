use crate::controllers::rabbitmq::rabbit_configuration::rabbit_configuration;
use crate::database::message_collection::insert_message;
use crate::model::message_model::Message;

pub fn message_manager(message: Message) {
    insert_message(Message {
        name: message.name.to_string(),
        create_at: message.create_at.to_string(),
    });
    rabbit_configuration(Message {
        name: message.name.to_string(),
        create_at: message.create_at.to_string(),
    });
}