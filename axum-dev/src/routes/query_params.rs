use axum::{extract::Query, Json};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// `Query` gives you the query parameters and deserializes them.
pub async fn query_params(Query(params): Query<HashMap<String, String>>) {
    for (key, value) in params {
        println!("{} = {}", key, value);
    }
}

#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    id: i32
}

pub async fn query_params_id(Query(query): Query<QueryParams>) -> Json<QueryParams> {  
    Json(query)
}