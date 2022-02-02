use isahc::{AsyncReadResponseExt, HttpClient};
use rand::{thread_rng, Rng};
use serenity::framework::standard::CommandError;

use super::nekoslife_image::NekosLifeImage;

const BASE_URL: &str = "https://nekos.life/api/v2/img";

pub struct NekosLifeClient {
    client: HttpClient,
}

impl NekosLifeClient {
    pub fn default() -> Self {
        let client = HttpClient::builder()
            .build()
            .expect("[API-N.L] Failed to create nekos.life client.");
        
        println!("[API-N.L] Nekos.life client created.");
        Self { client }
    }

    pub async fn get_cat(&self) -> Result<NekosLifeImage, CommandError> {
        let result = self.client.get_async(format!("{}/meow", BASE_URL)).await;

        match result {
            Ok(mut response) => Ok(response.json().await?),
            Err(_) => Err("[API-N.L] Failed to get cat image".into()),
        }
    }

    pub async fn gen_hentai(&self) -> Result<NekosLifeImage, CommandError> {
        let types = ["lewdk", "nsfw_neko_gif"];

        let num = thread_rng().gen_range(0..types.len());

        let result = self
            .client
            .get_async(format!("{}/{}", BASE_URL, types[num]))
            .await;

        match result {
            Ok(mut response) => Ok(response.json().await?),
            Err(_) => Err("[API-N.L] Failed to get hentai image".into()),
        }
    }
}