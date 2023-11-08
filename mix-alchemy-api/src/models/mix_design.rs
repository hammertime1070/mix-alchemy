use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MixDesign {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub location: String,
    // Add other fields as necessary
}

// Similarly, define other structures needed for interaction with mix designs
