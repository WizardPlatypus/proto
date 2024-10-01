use crate::routes::world;
use rocket::{Build, Rocket};

pub fn build() -> Rocket<Build> {
    rocket::build().mount("/hello", routes![world])
}
