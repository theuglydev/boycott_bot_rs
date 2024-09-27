use crate::models::api_response::ApiResponse;
use crate::models::brand::Brand;
use reqwest;
use serde_json;

pub async fn get_brands(
    brand: String,
) -> Result<Vec<Brand>, Box<dyn std::error::Error + Send + Sync>> {
    let mut brands: Vec<Brand> = Vec::new();
    let api_url: String = String::from("http://127.0.0.1:8080/fetchbrands?brand_name=") + &brand;

    let brands_res = reqwest::get(api_url).await;
    if brands_res.is_err() {
        let err_msg: String = brands_res.err().unwrap().to_string();
        return Err(err_msg.into());
    }

    let response = brands_res.unwrap();
    if response.status() != 200 {
        let err_msg: String = response.status().to_string();
        return Err(err_msg.into());
    }

    let text_extract = response.text().await;
    if text_extract.is_err() {
        let err_msg: String = text_extract.err().unwrap().to_string();
        return Err(err_msg.into());
    }

    let json_text = text_extract.unwrap();
    let api_response: ApiResponse = serde_json::from_str(&json_text)?;

    if api_response.status == 0 {
        let mut err_msg = String::new();
        if let Some(err) = api_response.err_msg {
            err_msg = err
        } else {
            err_msg = String::from("Unknown Error in api");
        }

        return Err(err_msg.into());
    }

    if let Some(data) = api_response.data {
        brands = data;
    } else {
        let err_msg: String = String::from("No data in api response");
        return Err(err_msg.into());
    }

    Ok(brands)
}
