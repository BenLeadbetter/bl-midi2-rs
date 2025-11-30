use std::collections::HashMap;
use std::sync::LazyLock;

pub static SCHEMA: LazyLock<Schema> =
    LazyLock::new(|| json5::from_str(include_str!("schema.json5")).unwrap());

#[derive(serde::Deserialize, Debug)]
pub struct Schema {
    pub types: Vec<String>,
    pub messages: HashMap<String, Message>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Message {
    pub name: String,
    pub ump: Option<Ump>,
    pub bytes: Option<Bytes>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Ump {
    pub packet_size: usize,
    pub max_packets: usize,
    pub min_packets: usize,
    pub properties: HashMap<String, UmpProperty>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Bytes {
    pub max_size: usize,
    pub min_size: usize,
    pub properties: HashMap<String, BytesProperty>,
}

#[derive(serde::Deserialize, Debug)]
pub struct UmpProperty {
    #[serde(rename = "type")]
    pub ty: String,
    pub constant: Option<u32>,
    pub packet_offset: usize,
    pub bit_offset: usize,
    pub length: usize,
}

#[derive(serde::Deserialize, Debug)]
pub struct BytesProperty {
    #[serde(rename = "type")]
    pub ty: String,
    pub constant: Option<u32>,
    pub bit_offset: usize,
    pub length: usize,
}
