#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;

mod player;
mod db_postgres;
mod service_hashing;
mod character;

#[post("/Player/Create", format = "json", data = "<user>")]
fn player_create(user: Json<player::UserCredentials>) -> String {

    println!("{:?}", user);

    let user = user.into_inner();

    if !player::user::exists(&user) {

        player::user::create(user)

    } else {

        let response = player::ErrorResponse {

            result_code : player::ResultCodes::UserAlreadyExists

        };
        serde_json::to_string(&response).unwrap()

    }
}

#[get("/Player/Login", format = "json", data = "<user>")]
fn player_login(user: Json<player::UserCredentials>) -> String {

    println!("{:?}", user);

    let user = user.into_inner();
    if player::user::exists(&user) {

        player::login_user::login(user)

    } else {

        let response = player::ErrorResponse {

            result_code : player::ResultCodes::UserDoesNotExist

        };
        serde_json::to_string(&response).unwrap()

    }

}

#[get("/Character/GetAll", format = "json", data = "<player>")]
fn get_characters(player: Json<player::Player>) -> String {

    character::get_characters(player.into_inner())

}

#[get("/Character/GetPosition", format = "json", data = "<character>")]
fn get_character_position(character: Json<character::CharacterId>) -> String {

    character::get_character_position(character.into_inner())

}

#[get("/Character/GetHealth", format = "json", data = "<character>")]
fn get_character_health(character: Json<character::CharacterId>) -> String {

    character::get_character_health(character.into_inner())

}

#[get("/")]
fn index() -> &'static str {

    "Hello, world!"

}

fn main() {

    rocket::ignite().mount("/", routes![index, player_create, player_login, get_characters, get_character_position, get_character_health]).launch();

}
