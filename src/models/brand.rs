use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Brand {
    pub brand_name: String,
    pub brand_image: String,
    pub proof: String,
    pub source: String,
}
