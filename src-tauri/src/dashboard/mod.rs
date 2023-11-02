use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Deserialize, Serialize)]
pub enum ChartType {
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "bar")]
    Bar,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub title: String,
    pub chart_type: ChartType,
    pub x_axis: String,
    pub query: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Text {
    pub template: String,
    pub variables: HashMap<String, String>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Component {
    Chart(Chart),
    Text(Text),
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Dashboard {
    pub name: String,
    pub components: Vec<Component>,
}
