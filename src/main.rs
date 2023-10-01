use rocket::{launch, routes};
use crate::views::index_view::{index, create_message};
mod views;
mod model;
mod database;
mod controllers;

#[launch]
fn rocket() -> _ {
   rocket::build().mount("/", routes![index, create_message])
}
