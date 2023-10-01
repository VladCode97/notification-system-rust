use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Debug)]
pub struct Message {
    pub name: String,
    pub create_at: String,
}