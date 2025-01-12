use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use crate::utils;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KubeListData {
    api_version: String,
    kind: String,
    metadata: HashMap<String, String>,
    items: Vec<KubeApiData>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KubeApiData {
    api_version: String,
    kind: String,
    metadata: KubeMetaData,
    spec: HashMap<String, Value>,
    status: HashMap<String, String>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KubeMetaData {
    name: String,
    labels: HashMap<String, String>,
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
