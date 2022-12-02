use std::{sync::Arc};

pub struct SharedState {
    pub is_admin: bool,
}

pub async fn get_role() -> Arc<SharedState> {
    Arc::new(SharedState {
        is_admin: true,
    })
}

