#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

struct ErrorResponse {

    error: String

}

#[post("/Person/CreateUser")]
fn person_create_user() -> String {


    let mut client = get_postgres_connection();
    match client {

        Ok(conn) => {

            println!("{}", "Sucessfully initialzed connection");
            "Hello".to_string()

        }
        Err(err) => {

            println!("{:?}", err);
            let error = ErrorResponse {

                error : err.to_string()

            };
            serde_json::to_string(&error).unwrap()

        }
    }
}

#[get("/Person/GetSession")]
fn person_get_session() -> String {
    let mut client = get_postgres_connection();
    match client {

        Ok(conn) => {

            println!("{}", "Sucessfully initialzed connection");
            "Hello".to_string()

        }
        Err(err) => {

            println!("{:?}", err);
            let error = ErrorResponse {

                error : err.to_string()

            };
            serde_json::to_string(&error).unwrap()

        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index, person_create_user, person_get_session]).launch();
}

fn get_postgres_connection() -> Result<(postgres::Client), postgres::Error> {

    let mut client = Client::connect("host=192.168.254.36 user=devtest password=devtest dbname=fivem", NoTls)?;
    Ok(client)

}