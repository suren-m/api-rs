use std::env;

use crate::dto::UpdateRequest;
use crate::error::Error;
use crate::schema::users;
use crate::schema::users::dsl::*;
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

pub fn create_user<'a>(req: SignupRequest, conn: &PgConnection) -> User {
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
    users.load::<User>(conn).expect("Error loading users")
}

pub fn get_user(loginreq_email: &str, conn: &PgConnection) -> User {
    users
        .filter(email.eq(loginreq_email))
        .first(conn)
        .expect("Error loading user")
}

pub fn get_user_by_id(user_id: i32, conn: &PgConnection) -> User {
    users.find(user_id).first(conn).expect("Error loading user")
}

pub fn user_exists(loginreq_email: &str, conn: &PgConnection) -> bool {
    match users.filter(email.eq(loginreq_email)).first::<User>(conn) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn update_user(user_id: i32, update_req: UpdateRequest, conn: &PgConnection) {
    diesel::update(users.find(user_id))
        .set((
            firstname.eq(update_req.firstname),
            lastname.eq(update_req.lastname),
        ))
        .get_result::<User>(conn)
        .expect(&format!("Unable to find user {}", user_id));
}
