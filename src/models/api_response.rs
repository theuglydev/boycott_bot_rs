use super::brand::Brand;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApiResponse {
    pub status: u32,
    pub err_msg: Option<String>,
    pub data: Option<Vec<Brand>>,
}
