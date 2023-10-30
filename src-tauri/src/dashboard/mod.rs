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
pub struct Dashboard {
    pub name: String,
    pub charts: Vec<Chart>,
}
