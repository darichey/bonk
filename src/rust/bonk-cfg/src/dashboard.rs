use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BarChart {
    pub title: String,
    pub x_axis: String,
    pub query: String,
    pub grid_column: String,
    pub grid_row: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LineChart {
    pub title: String,
    pub x_axis: String,
    pub query: String,
    pub grid_column: String,
    pub grid_row: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub template: String,
    pub variables: HashMap<String, String>,
    pub grid_column: String,
    pub grid_row: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub title: String,
    pub query: String,
    pub grid_column: String,
    pub grid_row: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Component {
    BarChart(BarChart),
    LineChart(LineChart),
    Text(Text),
    Table(Table),
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Dashboard {
    pub name: String,
    pub components: Vec<Component>,
}

pub fn from_toml(src: &str) -> Result<Dashboard, toml::de::Error> {
    toml::from_str(src)
}
