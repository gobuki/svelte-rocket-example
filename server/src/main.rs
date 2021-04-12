#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::error::Error as RocketError;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::Json;
use rocket::{ State };
use rocket::response::{ Responder };
use rocket::request::{ FromRequest, Request };


async fn rocket() -> Result<rocket::Rocket, LaunchError> {
    let app = rocket::ignite()
             .mount("/",StaticFiles::from("../client/svelte/app/__sapper__/export"));
    Ok(app) 
}

#[rocket::main]
async fn main() -> Result<(), LaunchError> {
    rocket().await?.launch().await?; 
    Ok(())
}

#[derive(Debug)]
enum LaunchError {
    TODO
}


impl From<RocketError> for LaunchError {
    fn from(_: RocketError) -> Self {
        LaunchError::TODO
    }
}
