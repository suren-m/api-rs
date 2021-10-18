extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use api_rs::{error, handlers::*};
use diesel_migrations::embed_migrations;

// https://docs.rs/diesel/0.16.0/diesel/macro.embed_migrations.html
embed_migrations!();

use warp::Filter;

use api_rs::auth::with_auth;

use api_rs::repository::establish_connection;

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
