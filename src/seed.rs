use std::collections::HashMap;

use crate::{dto::SignupRequest, models::User, repository::establish_connection};

pub fn init_users() -> HashMap<i32, User> {
    let mut map = HashMap::new();
    let conn = establish_connection();

    let user1 = SignupRequest {
        firstname: String::from("john"),
        lastname: String::from("denver"),
        email: String::from("user@userland.com"),
        password: String::from("1234"),
    };

    let user2 = SignupRequest {
        firstname: String::from("jane"),
        lastname: String::from("doe"),
        email: String::from("admin@adminaty.com"),
        password: String::from("4321"),
    };

    //let user1 = create_user(&conn, user1);
    //let user2 = create_user(&conn, user2);

    //map.insert(user1.id, user1);
    //map.insert(user2.id, user2);
    map
}
