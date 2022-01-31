use isahc::{AsyncReadResponseExt, HttpClient};
// use rand::{thread_rng, Rng};
use serenity::framework::standard::CommandError;

use super::lunardev_image::LunarDevImage;

const BASE_URL: &str = "https://lunardev.group/api/";

pub struct LunarDevClient {
    client: HttpClient,
}

impl LunarDevClient {
    pub fn default() -> Self {
        let client = HttpClient::builder()
            .build()
            .expect("[API-L.D] Failed to create LunarDev client.");
        
        println!("[API-L.D] LunarDev client created.");
        Self { client }
    }

    pub async fn gen_neko(&self) -> Result<LunarDevImage, CommandError> {
        let result = self.client.get_async(format!("{}/neko", BASE_URL)).await;

        match result {
            Ok(mut response) => Ok(response.json().await?),
            Err(_) => Err("[API-L.D] Failed to get neko image".into()),
        }
    }
}