use warp::{reject, reply, Reply};

use crate::error::Error::*;
use crate::{auth, crypto::verify_hash, dto::*, repository::*, WebResult};

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
