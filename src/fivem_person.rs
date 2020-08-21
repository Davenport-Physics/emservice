
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ResultCodes {

    Successful,
    UserAlreadyExists,
    GeneralError,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCredentials {

    pub username: String,
    pub password_hash: String

}
#[derive(Serialize, Deserialize, Debug)]
struct UserSession {

    session_id: String,
    result_code: ResultCodes

}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {

    pub result_code: ResultCodes

}


pub mod create_user {

    use crate::db_postgres;
    use crate::fivem_person::{UserCredentials, UserSession, ErrorResponse, ResultCodes};
    use crate::service_hashing;

    pub fn create(user: UserCredentials) -> String {

        let client = db_postgres::get_connection();
        match client {

            Ok(conn) => {

                let session_id   = user_set(conn, user);
                let user_session = UserSession {

                    session_id: session_id,
                    result_code: ResultCodes::Successful

                };
                serde_json::to_string(&user_session).unwrap()

            }
            Err(err) => {

                println!("{:?}", err);
                let error = ErrorResponse {

                    result_code: ResultCodes::GeneralError

                };
                serde_json::to_string(&error).unwrap()
            }

        }

    }

    pub fn user_exists(user: &UserCredentials) -> bool {

        let client = db_postgres::get_connection();

        match client {

            Ok(mut client) => {

                let row = client.query_one("SELECT * FROM Person.User WHERE UserName = $1", &[&user.username]);
                match row {
                    Ok(_) => true,
                    Err(err) => {

                        println!("{:?}", err);
                        false
                        
                    }
                }
            }
            Err(err) => {

                println!("{:?}", err);
                false

            }
        }
    }

    fn user_set(mut client: postgres::Client, user: UserCredentials) -> String {

        let argon2_hash = service_hashing::get_argon2_hash(&user.password_hash);

        let row = client.query_one("INSERT INTO Person.User (UserName, PasswordHash, PasswordSalt) VALUES ($1, $2, $3) RETURNING UserId", &[&user.username, &argon2_hash.hash, &argon2_hash.salt]).unwrap();
        let user_id: i32 = row.get("UserId");

        let argon2_hash = service_hashing::get_argon2_hash(&user.password_hash);
        let session_id  = service_hashing::get_sha512_hash(&argon2_hash.hash);

        client.execute("INSERT INTO Person.UserSession (UserId, SessionId) VALUES ($1, $2)", &[&user_id, &session_id]).unwrap();

        session_id

    }
}

pub mod login_user {

    use crate::db_postgres;
    use crate::fivem_person::{UserCredentials, UserSession};
    use crate::service_hashing;

    pub fn login(user: UserCredentials) -> String {

        "".to_string()

    }

}