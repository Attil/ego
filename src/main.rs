#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/<game>")]
fn game(game: String) -> String {
    format!("This is game: {}!", game)
}

fn main() {
    rocket::ignite().mount("/game", routes![game]).launch();
}