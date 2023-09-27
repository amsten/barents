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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AISStaticData {
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
    pub draught: Option<i32>,
    pub ship_length: Option<i32>,
    pub ship_width: Option<i32>,
    pub ship_type: Option<i32>,
    pub dimension_a: Option<i32>,
    pub dimension_b: Option<i32>,
    pub dimension_c: Option<i32>,
    pub dimension_d: Option<i32>,
    pub position_fixing_device_type: Option<i64>,
    pub report_class: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AISAtonData {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub message_type: Option<i64>,
    pub mmsi: Option<i64>,
    pub msgtime: Option<String>,
    pub dimension_a: Option<i32>,
    pub dimension_b: Option<i32>,
    pub dimension_c: Option<i32>,
    pub dimension_d: Option<i32>,
    pub type_of_aids_to_navigation: Option<i64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub name: Option<String>,
    pub type_of_electronic_fixing_device: Option<i64>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AISPositionData {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub message_type: Option<i64>,
    pub course_over_ground: Option<f64>,
    pub ais_class: Option<String>,
    pub altitude: Option<f64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub navigational_status: Option<i64>,
    pub rate_of_turn: Option<i64>,
    pub speed_over_ground: Option<f64>,
    pub true_heading: Option<i64>,
    pub mmsi: Option<i64>,
    pub msgtime: Option<String>,
}

pub type AISLatestResponses = Vec<GetAISLatestResponseItem>;
#[derive(Default)]
pub struct GetAISLatestResponse {
    pub api_endpoint: String,
    pub status_code: u16,
    pub content_length: Option<usize>,
    pub ais_latest_responses: Option<AISLatestResponses>,
}

//noinspection ALL
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAISLatestResponseItem {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub message_type: Option<i64>,
    pub mmsi: Option<i64>,
    pub msgtime: Option<String>,

    // Fields from AISStaticData
    pub imo_number: Option<i64>,
    pub call_sign: Option<String>,
    pub destination: Option<String>,
    pub eta: Option<String>,
    pub name: Option<String>,
    pub draught: Option<i32>,
    pub ship_length: Option<i32>,
    pub ship_width: Option<i32>,
    pub ship_type: Option<i32>,
    pub dimension_a: Option<i32>,
    pub dimension_b: Option<i32>,
    pub dimension_c: Option<i32>,
    pub dimension_d: Option<i32>,
    pub position_fixing_device_type: Option<i64>,
    pub report_class: Option<String>,

    // Fields from AISAtonData
    pub type_of_aids_to_navigation: Option<i64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub type_of_electronic_fixing_device: Option<i64>,

    // Fields from AISPositionData
    pub course_over_ground: Option<f64>,
    pub ais_class: Option<String>,
    pub altitude: Option<f64>,
    pub navigational_status: Option<i64>,
    pub rate_of_turn: Option<i64>,
    pub speed_over_ground: Option<f64>,
    pub true_heading: Option<i64>,
}

impl From<&GetAISLatestResponseItem> for AISPositionData {
    fn from(item: &GetAISLatestResponseItem) -> Self {
        AISPositionData {
            type_field: item.type_field.clone(),
            message_type: item.message_type,
            course_over_ground: item.course_over_ground,
            ais_class: item.ais_class.clone(),
            altitude: item.altitude,
            latitude: item.latitude,
            longitude: item.longitude,
            navigational_status: item.navigational_status,
            rate_of_turn: item.rate_of_turn,
            speed_over_ground: item.speed_over_ground,
            true_heading: item.true_heading,
            mmsi: item.mmsi,
            msgtime: item.msgtime.clone(),
        }
    }
}

impl From<&GetAISLatestResponseItem> for AISStaticData {
    fn from(item: &GetAISLatestResponseItem) -> Self {
        AISStaticData {
            type_field: item.type_field.clone(),
            message_type: item.message_type,
            mmsi: item.mmsi,
            msgtime: item.msgtime.clone(),
            imo_number: item.imo_number,
            call_sign: item.call_sign.clone(),
            destination: item.destination.clone(),
            eta: item.eta.clone(),
            name: item.name.clone(),
            draught: item.draught,
            ship_length: item.ship_length,
            ship_width: item.ship_width,
            ship_type: item.ship_type,
            dimension_a: item.dimension_a,
            dimension_b: item.dimension_b,
            dimension_c: item.dimension_c,
            dimension_d: item.dimension_d,
            position_fixing_device_type: item.position_fixing_device_type,
            report_class: item.report_class.clone(),
        }
    }
}

impl From<&GetAISLatestResponseItem> for AISAtonData {
    fn from(item: &GetAISLatestResponseItem) -> Self {
        AISAtonData {
            type_field: item.type_field.clone(),
            message_type: item.message_type,
            mmsi: item.mmsi,
            msgtime: item.msgtime.clone(),
            dimension_a: item.dimension_a,
            dimension_b: item.dimension_b,
            dimension_c: item.dimension_c,
            dimension_d: item.dimension_d,
            type_of_aids_to_navigation: item.type_of_aids_to_navigation,
            latitude: item.latitude,
            longitude: item.longitude,
            name: item.name.clone(),
            type_of_electronic_fixing_device: item.type_of_electronic_fixing_device,
        }
    }
}
