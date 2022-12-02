#[derive(Clone)]
pub struct SharedData{
    pub message: String,
}

pub async fn shared_message_data() -> SharedData {
    SharedData {
        message: "Hello from Shared Data".to_owned(),
    }
}