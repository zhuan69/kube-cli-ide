use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use crate::utils;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KubeListData {
    pub api_version: String,
    pub kind: String,
    pub metadata: HashMap<String, Value>,
    pub items: Vec<KubeApiData>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KubeApiData {
    pub api_version: String,
    pub kind: String,
    pub metadata: KubeMetaData,
    pub spec: HashMap<String, Value>,
    pub status: HashMap<String, Value>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KubeMetaData {
    pub name: String,
    pub labels: HashMap<String, Value>,
}

impl KubeListData {
    pub fn new_list_api_data_from_string(json_string: &str) -> Option<Self> {
        utils::serde_string_to_json::<KubeListData>(json_string)
    }
}

impl KubeApiData {
    pub fn new_api_data_from_string(json_string: &str) -> Option<Self> {
        utils::serde_string_to_json::<KubeApiData>(json_string)
    }
}
