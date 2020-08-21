#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;
mod fivem_person;
mod db_postgres;
mod service_hashing;

#[post("/Person/CreateUser", format = "json", data = "<user>")]
fn person_create_user(user: Json<fivem_person::UserCredentials>) -> String {

    println!("{:?}", user);

    let user = user.into_inner();

    if !fivem_person::create_user::user_exists(&user) {

        fivem_person::create_user::create(user)

    } else {

        let response = fivem_person::ErrorResponse {

            result_code : fivem_person::ResultCodes::UserAlreadyExists

        };
        serde_json::to_string(&response).unwrap()

    }
}

#[get("/Person/Login", format = "json", data = "<user>")]
fn person_get_session(user: Json<fivem_person::UserCredentials>) -> String {

    println!("{:?}", user);

    let user = user.into_inner();
    if fivem_person::create_user::user_exists(&user) {

        fivem_person::login_user::login(user)

    } else {

        let response = fivem_person::ErrorResponse {

            result_code : fivem_person::ResultCodes::UserDoesNotExist

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
