#[derive(Clone)]
pub struct SharedData{
    pub message: String,
}

pub async fn message() -> SharedData {
    SharedData {
        message: "Hello from Shared Data".to_owned(),
    }
}