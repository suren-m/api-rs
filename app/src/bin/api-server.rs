extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use diesel_migrations::embed_migrations;

// https://docs.rs/diesel/0.16.0/diesel/macro.embed_migrations.html
embed_migrations!();

use warp::{reject, reply, Filter, Reply};

use api_rs::auth::{self, with_auth};
use api_rs::crypto::verify_hash;
use api_rs::dto::{
    LoginRequest, LoginResponse, SignupRequest, UpdateRequest, UserResponse, UsersResponse,
};
use api_rs::error::{self, Error::*};
use api_rs::repository::{
    create_user, establish_connection, get_user, get_users, update_user, user_exists,
};
use api_rs::WebResult;

#[tokio::main]
async fn main() {
    println!("Establishing Connection");
    let conn = establish_connection();

    println!("Running Migrations");
    let _ = embedded_migrations::run(&conn).expect("unable to run migrations");

    let signup_route = warp::path!("signup")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(signup_handler);

    let login_route = warp::path!("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(login_handler);

    let users_route = warp::path!("users")
        .and(warp::get())
        .and(with_auth())
        .and_then(users_handler);

    let update_route = warp::path!("users")
        .and(warp::put())
        .and(with_auth())
        .and(warp::body::json())
        .and_then(update_handler);

    let routes = signup_route
        .or(login_route)
        .or(users_route)
        .or(update_route)
        .recover(error::handle_rejection);

    println!("listening on port 8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

pub async fn signup_handler(body: SignupRequest) -> WebResult<impl Reply> {
    println!("Signup Request");
    let conn = establish_connection();
    if user_exists(&body.email, &conn) {
        return Err(reject::custom(UserAlreadyExistsError));
    };
    let user = create_user(body, &conn);
    let token = auth::create_jwt(&user.id).map_err(|e| reject::custom(e))?;
    Ok(reply::json(&LoginResponse { token }))
}

pub async fn login_handler(body: LoginRequest) -> WebResult<impl Reply> {
    println!("Login Request");
    let conn = establish_connection();
    let user = get_user(&body.email, &conn);

    let is_password_valid = verify_hash(&body.password, &user.password);

    if user.email == body.email && is_password_valid {
        let token = auth::create_jwt(&user.id).map_err(|e| reject::custom(e))?;
        Ok(reply::json(&LoginResponse { token }))
    } else {
        Err(reject::custom(WrongCredentialsError))
    }
}

pub async fn users_handler(_id: i32) -> WebResult<impl Reply> {
    println!("Users Request");
    let conn = establish_connection();
    let db_users = get_users(&conn);
    let mut users: Vec<UserResponse> = vec![];
    for db_user in db_users {
        let user = UserResponse {
            firstname: db_user.firstname,
            lastname: db_user.lastname,
            email: db_user.email,
        };
        users.push(user);
    }

    Ok(reply::json(&UsersResponse { users }))
}

pub async fn update_handler(id: i32, body: UpdateRequest) -> WebResult<impl Reply> {
    println!("Update Request");
    let conn = establish_connection();
    update_user(id, body, &conn);
    Ok(reply())
}