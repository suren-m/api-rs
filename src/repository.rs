use std::env;

use crate::schema::users;
use crate::{
    dto::SignupRequest,
    models::{NewUser, User},
};

use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn create_user<'a>(conn: &PgConnection, req: SignupRequest) -> User {
    let new_user = NewUser {
        firstname: &req.firstname,
        lastname: &req.lastname,
        email: &req.email,
        password: &req.password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn get_users(conn: &PgConnection) -> Vec<User> {
    use crate::schema::users::dsl::*;
    users.load::<User>(conn).expect("Error loading users")
}

pub fn get_user(loginreq_email: &str, conn: &PgConnection) -> User {
    use crate::schema::users::dsl::*;
    users
        .filter(email.eq(loginreq_email))
        .first(conn)
        .expect("Error loading user")
}
