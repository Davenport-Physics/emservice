#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;

mod service_hashing;
mod fivem_person;
mod db_postgres;

#[post("/Person/CreateUser", format = "json", data = "<user>")]
fn person_create_user(user: Json<fivem_person::UserCredentials>) -> String {

    println!("{:?}", user);
    fivem_person::create_user::create(user.into_inner())

}

#[get("/Person/GetSession")]
fn person_get_session() -> String {

    "".to_string()

}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index, person_create_user, person_get_session]).launch();
}
