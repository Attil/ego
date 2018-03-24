#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate rocket;

mod database;
mod game;
mod question;

use game::Game;

#[get("/start")]
fn game() -> String {

    format!("This is game!")
}

fn main() {
    rocket::ignite().mount("/game", routes![game]).launch();
}