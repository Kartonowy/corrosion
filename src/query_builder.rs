use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use std::fmt::Debug;

pub async fn build<T>(url: String) -> Result<T, StatusCode>
where
    T: DeserializeOwned + Debug,
{
    let response = reqwest::get(url).await;

    match &response {
        Ok(r) => {
            if r.status() != StatusCode::OK {
                return Err(r.status());
            }
        }
        Err(e) => {
            if e.is_status() {
                return Err(e.status().unwrap());
            } else {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }

    let content = response.unwrap().json::<T>().await;

    match content {
        Ok(s) => Ok(s),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

/* pub async fn all<T>(call: &str) -> Result<T, StatusCode> {


} */
