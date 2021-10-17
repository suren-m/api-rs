use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}
