use api_rs::auth::{self, with_auth};
use api_rs::dto::{LoginRequest, LoginResponse, SignupRequest};
use api_rs::error::{self, Error::*};
use api_rs::repository::{establish_connection, get_user, get_users};
use api_rs::WebResult;
use warp::{reject, reply, Filter, Reply};

#[tokio::main]
async fn main() {
    let conn = establish_connection();
    let _ = get_users(&conn);

    let signup_route = warp::path!("signup")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(signup_handler);

    let login_route = warp::path!("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(login_handler);

    let user_route = warp::path!("user").and(with_auth()).and_then(user_handler);

    let routes = signup_route
        .or(login_route)
        .or(user_route)
        .recover(error::handle_rejection);

    println!("listening on port 8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

pub async fn login_handler(body: LoginRequest) -> WebResult<impl Reply> {
    let conn = establish_connection();
    let user = get_user(&body.email, &conn);

    if user.email == body.email && user.password == body.password {
        let token = auth::create_jwt(&user.id).map_err(|e| reject::custom(e))?;
        Ok(reply::json(&LoginResponse { token }))
    } else {
        Err(reject::custom(WrongCredentialsError))
    }
}

pub async fn user_handler(id: i32) -> WebResult<impl Reply> {
    todo!();
    Ok(format!("Hello User {}", id))
}

pub async fn signup_handler(body: SignupRequest) -> WebResult<impl Reply> {
    todo!();
    Ok(format!("Hello User {}", body.firstname.to_owned()))
}
