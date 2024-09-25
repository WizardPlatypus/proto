use rocket::{Rocket, Build};
use crate::routes::world;

pub fn build() -> Rocket<Build>{
    rocket::build()
        .mount("/hello", routes![world])
}
