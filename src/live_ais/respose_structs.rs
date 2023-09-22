use serde::Deserialize;
use serde::Serialize;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
    #[serde(rename = "token_type")]
    pub token_type: String,
    pub scope: String,
}

pub type GetAISLatestResponses = Vec<GetAISLatestResponseItem>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAISLatestResponseItem {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub message_type: Option<i64>,
    pub mmsi: Option<i64>,
    pub msgtime: Option<String>,
    pub imo_number: Option<i64>,
    pub call_sign: Option<String>,
    pub destination: Option<String>,
    pub eta: Option<String>,
    pub name: Option<String>,
    pub draught: Option<i64>,
    pub ship_length: Option<i64>,
    pub ship_width: Option<i64>,
    pub ship_type: Option<i64>,
    pub dimension_a: Option<i64>,
    pub dimension_b: Option<i64>,
    pub dimension_c: Option<i64>,
    pub dimension_d: Option<i64>,
    pub position_fixing_device_type: Option<i64>,
    pub report_class: Option<String>,
}

