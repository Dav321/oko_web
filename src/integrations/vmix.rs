use std::collections::BTreeMap;
#[cfg(feature = "server")]
use crate::backend::CLIENT;
#[cfg(feature = "server")]
use rusqlite::ToSql;
#[cfg(feature = "server")]
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use serde_xml_rs::from_str;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Overlay {
    Overlay1 = 1,
    Overlay2 = 2,
    Overlay3 = 3,
    Overlay4 = 4,
    Overlay5 = 5,
    Overlay6 = 6,
    Overlay7 = 7,
    Overlay8 = 8,
}

impl FromStr for Overlay {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Overlay::Overlay1),
            "2" => Ok(Overlay::Overlay2),
            "3" => Ok(Overlay::Overlay3),
            "4" => Ok(Overlay::Overlay4),
            "5" => Ok(Overlay::Overlay4),
            "6" => Ok(Overlay::Overlay4),
            "7" => Ok(Overlay::Overlay4),
            "8" => Ok(Overlay::Overlay4),
            &_ => Err("Unknown overlay type"),
        }
    }
}

#[cfg(feature = "server")]
impl ToSql for Overlay {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.clone() as u8))
    }
}

#[cfg(feature = "server")]
impl FromSql for Overlay {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match u8::column_result(value)? {
            1 => Ok(Overlay::Overlay1),
            2 => Ok(Overlay::Overlay2),
            3 => Ok(Overlay::Overlay3),
            4 => Ok(Overlay::Overlay4),
            5 => Ok(Overlay::Overlay5),
            6 => Ok(Overlay::Overlay6),
            7 => Ok(Overlay::Overlay7),
            8 => Ok(Overlay::Overlay8),
            i => Err(FromSqlError::OutOfRange(i as i64)),
        }
    }
}

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
    key: String,
    r#type: String,
    title: String,
    text: Option<Vec<Text>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Text {
    name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vmix {
    vmix_url: String,

    overlay: Overlay,
    object_uuid: String,
}

impl Vmix {
    pub fn new(vmix_url: String, overlay: Overlay, object_uuid: String) -> Result<Self, String> {
        Ok(Self {
            vmix_url,
            overlay,
            object_uuid,
        })
    }

    pub fn get_vmix_url(&self) -> String {
        self.vmix_url.clone()
    }

    pub fn get_overlay_index(&self) -> Overlay {
        self.overlay
    }

    pub fn get_object_uuid(&self) -> String {
        self.object_uuid.clone()
    }

    #[cfg(feature = "server")]
    pub async fn get_vmix_titles(&self) -> Result<BTreeMap<String, (String, Vec<String>)>, String> {
        let info = CLIENT.with(|client| {
            client.get("http://".to_owned() + &self.vmix_url + "/api/")
                .send()
        }).await;
        
        let info = match info {
            Ok(result) => result,
            Err(e) => return Err(e.to_string()),
        };

        if !info.status().is_success() {
            return Err(info.status().to_string());
        }
        
        let raw = info.text().await.unwrap();
        
        let xml: VmixXML = from_str(&raw).unwrap();
        
        let options: Vec<&Input> = xml.inputs.inputs.iter()
            .filter(|input| input.r#type == "GT")
            .collect();
        
        let mut res = BTreeMap::new();
        for option in options {
            if let Some(texts) = &option.text {
                let texts: Vec<String> = texts.iter().map(|text| {
                    text.name.clone()
                }).collect();
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
                    .get("http://".to_owned() + &self.vmix_url + "/api/")
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
                &format!("{}{}", "OverlayInput", self.overlay as u8),
            ),
            ("Input", &self.object_uuid.to_string()),
        ])
        .await
    }
}
