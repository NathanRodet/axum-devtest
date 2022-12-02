use axum::Json;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct UserTest {
    pub username: String,
    pub age: u32,
}

pub async fn get_json() -> Json<UserTest> {
    Json(UserTest {
        username: "John".to_owned(),
        age: 30,
    })  
}