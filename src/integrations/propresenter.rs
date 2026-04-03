#[cfg(feature = "server")]
use crate::backend::CLIENT;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProPresenter {
    url: String,

    message_name: String,

    theme_name: String,
    theme_index: String,
    theme_uuid: String,
}

pub struct Theme {
    id: ThemeId,
}

pub struct ThemeId {
    index: i32,
    name: String,
    uuid: String,
}

impl ProPresenter {
    pub fn new(
        pro_presenter_url: String,
        message_name: String,
        theme_name: String,
        theme_index: String,
        theme_uuid: String,
    ) -> Self {
        Self {
            url: pro_presenter_url,
            message_name,
            theme_name,
            theme_index,
            theme_uuid,
        }
    }

    pub fn get_pro_presenter_url(&self) -> String {
        self.url.clone()
    }

    pub fn get_message_name(&self) -> String {
        self.message_name.clone()
    }

    pub fn get_theme_name(&self) -> String {
        self.theme_name.clone()
    }

    pub fn get_theme_index(&self) -> String {
        self.theme_index.clone()
    }

    pub fn get_theme_uuid(&self) -> String {
        self.theme_uuid.clone()
    }

    #[cfg(feature = "server")]
    pub async fn add_message(&self, message: String) -> Result<(), String> {
        let url = "http://".to_owned() + &self.url + "/v1/messages";

        let payload = json!({
            "id": json!({"name": self.message_name}),
            "message": message,
            "theme": json!({
                "name": self.theme_name,
                "index": i32::from_str(&self.theme_index).unwrap(),
                "uuid": self.theme_uuid,
            }),
            "tokens": Vec::<i32>::new(),
            "visible_on_network": true,
            "is_active": true,
        });

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());

        let response = CLIENT
            .with(|client| client.post(url).headers(headers).json(&payload).send())
            .await;

        let result = match response {
            Ok(result) => result,
            Err(e) => return Err(e.to_string()),
        };

        if !result.status().is_success() {
            return Err(result.text().await.unwrap());
        }

        Ok(())
    }

    #[cfg(feature = "server")]
    pub async fn edit_message(&self, message: String) -> Result<(), String> {
        let url = "http://".to_owned() + &self.url + "/v1/message/" + &self.message_name;

        let payload = json!({
            "id": json!({"name": self.message_name}),
            "message": message,
            "theme": json!({
                "name": self.theme_name,
                "index": self.theme_index,
                "uuid": self.theme_uuid,
            }),
            "tokens": Vec::<i32>::new(),
            "visible_on_network": true,
        });

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());

        let response = CLIENT
            .with(|client| client.put(url).headers(headers).json(&payload).send())
            .await;

        let result = match response {
            Ok(result) => result,
            Err(e) => return Err(e.to_string()),
        };

        if !result.status().is_success() {
            return Err(result.text().await.unwrap());
        }

        Ok(())
    }

    #[cfg(feature = "server")]
    pub async fn trigger_message(&self) -> Result<(), String> {
        let url =
            "http://".to_owned() + &self.url + "/v1/message/" + &self.message_name + "/trigger";

        let payload = json!(Vec::<i32>::new());

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());

        let response = CLIENT
            .with(|client| client.post(url).headers(headers).json(&payload).send())
            .await;

        let result = match response {
            Ok(result) => result,
            Err(e) => return Err(e.to_string()),
        };

        if !result.status().is_success() {
            return Err(result.text().await.unwrap());
        }

        Ok(())
    }

    #[cfg(feature = "server")]
    pub async fn clear_message(&self) -> Result<(), String> {
        let url = "http://".to_owned() + &self.url + "/v1/message/" + &self.message_name + "/clear";

        let response = CLIENT.with(|client| client.get(url).send()).await;

        let result = match response {
            Ok(result) => result,
            Err(e) => return Err(e.to_string()),
        };

        if !result.status().is_success() {
            return Err(result.text().await.unwrap());
        }

        Ok(())
    }

    #[cfg(feature = "server")]
    pub async fn remove_message(&self) -> Result<(), String> {
        let url = "http://".to_owned() + &self.url + "/v1/message/" + &self.message_name;

        let response = CLIENT.with(|client| client.delete(url).send()).await;

        let result = match response {
            Ok(result) => result,
            Err(e) => return Err(e.to_string()),
        };

        if !result.status().is_success() && result.status() != StatusCode::NOT_FOUND {
            return Err(result.text().await.unwrap());
        }

        Ok(())
    }
}
