use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Material {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub price: f64,
    // Add other fields as necessary
}

// Similarly, define other structures needed for interaction with materials