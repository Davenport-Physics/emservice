#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;
mod player;
mod db_postgres;
mod service_hashing;

#[post("/Player/Create", format = "json", data = "<user>")]
fn person_create_user(user: Json<player::UserCredentials>) -> String {

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
fn person_get_session(user: Json<player::UserCredentials>) -> String {

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

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index, person_create_user, person_get_session]).launch();
}
