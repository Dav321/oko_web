#[cfg(feature = "server")]
use crate::backend::CLIENT;
use serde::{Deserialize, Deserializer, Serialize};
use serde_xml_rs::from_str;
use std::collections::BTreeMap;
use std::str;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VmixXML {
    version: String,
    edition: String,

    inputs: Inputs,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Inputs {
    #[serde(rename = "input", default)]
    inputs: Vec<Input>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Input {
    #[serde(rename = "@key")]
    key: String,
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "@title")]
    title: String,
    text: Option<Vec<Text>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Text {
    #[serde(rename = "@name")]
    name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vmix {
    url: String,

    overlay_index: String,
    object_uuid: String,

    name_field: String,
    title_field: String,
}

impl Vmix {
    pub fn new(
        vmix_url: String,
        overlay: String,
        object_uuid: String,
        name_field: String,
        title_field: String,
    ) -> Result<Self, &'static str> {
        let overlay_num = match u8::from_str(&overlay) {
            Ok(n) => n,
            Err(_) => return Err("Overlay NaN"),
        };
        if overlay_num > 8 || overlay_num < 1 {
            return Err("Invalid overlay!");
        }
        Ok(Self {
            url: vmix_url,
            overlay_index: overlay,
            object_uuid,
            name_field,
            title_field,
        })
    }

    pub fn get_vmix_url(&self) -> String {
        self.url.clone()
    }

    pub fn get_overlay_index(&self) -> String {
        self.overlay_index.clone()
    }

    pub fn get_object_uuid(&self) -> String {
        self.object_uuid.clone()
    }

    pub fn get_name_field(&self) -> String {
        self.name_field.clone()
    }

    pub fn get_title_field(&self) -> String {
        self.title_field.clone()
    }

    #[cfg(feature = "server")]
    pub async fn get_vmix_titles(&self) -> Result<BTreeMap<String, (String, Vec<String>)>, String> {
        let info = CLIENT
            .with(|client| {
                client
                    .get("http://".to_owned() + &self.url + "/api/")
                    .send()
            })
            .await;

        let info = match info {
            Ok(result) => result,
            Err(e) => return Err(e.to_string()),
        };

        if !info.status().is_success() {
            return Err(info.status().to_string());
        }

        let raw = info.text().await.unwrap();

        let xml: VmixXML = from_str(&raw).unwrap();

        let options: Vec<&Input> = xml
            .inputs
            .inputs
            .iter()
            .filter(|input| input.r#type == "GT")
            .collect();

        let mut res = BTreeMap::new();
        for option in options {
            if let Some(texts) = &option.text {
                let texts: Vec<String> = texts.iter().map(|text| text.name.clone()).collect();
                res.insert(option.key.clone(), (option.title.clone(), texts));
            }
        }

        Ok(res)
    }

    #[cfg(feature = "server")]
    async fn request(&self, args: Vec<(&str, &str)>) -> Result<(), String> {
        let response = CLIENT
            .with(|client| {
                client
                    .get("http://".to_owned() + &self.url + "/api/")
                    .query(&args)
                    .send()
            })
            .await;

        let result = match response {
            Ok(result) => result,
            Err(e) => return Err(e.to_string()),
        };

        if !result.status().is_success() {
            return Err(result.status().to_string());
        }

        Ok(())
    }

    #[cfg(feature = "server")]
    pub async fn set_text(&self, value: String, field: String) -> Result<(), String> {
        self.request(vec![
            ("Function", "SetText"),
            ("Input", &self.object_uuid.to_string()),
            ("SelectedName", &field),
            ("Value", &value),
        ])
        .await
    }

    #[cfg(feature = "server")]
    pub async fn overlay_input(&self) -> Result<(), String> {
        self.request(vec![
            (
                "Function",
                &format!("{}{}", "OverlayInput", self.overlay_index),
            ),
            ("Input", &self.object_uuid.to_string()),
        ])
        .await
    }
}
