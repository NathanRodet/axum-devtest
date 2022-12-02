use axum::extract::Path;

// `Path` gives you the path parameters and deserializes them.
pub async fn get_path_variables(Path(user_id): Path<u32>) {
    println!("user_id = {}", user_id);
}