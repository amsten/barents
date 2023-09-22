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
    pub type_field: String,
    pub message_type: i64,
    pub mmsi: i64,
    pub msgtime: String,
    pub dimension_a: Option<i64>,
    pub dimension_b: Option<i64>,
    pub dimension_c: Option<i64>,
    pub dimension_d: Option<i64>,
    pub type_of_aids_to_navigation: Option<i64>,
    pub latitude: Option<i64>,
    pub longitude: Option<i64>,
    pub name: Option<String>,
    pub type_of_electronic_fixing_device: Option<i64>,
    pub functional_id: Option<i64>,
    pub designated_area_code: Option<i64>,
    pub day: Option<i64>,
    pub hour: Option<i64>,
    pub minute: Option<i64>,
    pub avg_wind_speed: Option<i64>,
    pub wind_gust: Option<i64>,
    pub wind_direction: Option<i64>,
    pub wind_gust_direction: Option<i64>,
    pub air_temperature: Option<i64>,
    pub relative_humidity: Option<i64>,
    pub dew_point: Option<i64>,
    pub air_pressure: Option<i64>,
    pub air_pressure_tendency: Option<String>,
    pub horizontal_visibility: Option<i64>,
    pub water_level: Option<i64>,
    pub water_level_trend: Option<String>,
    pub surface_current_speed: Option<i64>,
    pub surface_current_direction: Option<i64>,
    pub current_speed2: Option<i64>,
    pub current_direction2: Option<i64>,
    pub current_measuring_level2: Option<i64>,
    pub current_speed3: Option<i64>,
    pub current_direction3: Option<i64>,
    pub current_measuring_level3: Option<i64>,
    pub significant_wave_height: Option<i64>,
    pub wave_period: Option<i64>,
    pub wave_direction: Option<i64>,
    pub swell_height: Option<i64>,
    pub swell_period: Option<i64>,
    pub swell_direction: Option<i64>,
    pub sea_state: Option<String>,
    pub water_temperature: Option<i64>,
    pub precipitation_type: Option<String>,
    pub salinity: Option<i64>,
    pub ice: Option<String>,
    pub course_over_ground: Option<i64>,
    pub ais_class: Option<String>,
    pub altitude: Option<i64>,
    pub navigational_status: Option<i64>,
    pub rate_of_turn: Option<i64>,
    pub speed_over_ground: Option<i64>,
    pub true_heading: Option<i64>,
    pub imo_number: Option<i64>,
    pub call_sign: Option<String>,
    pub destination: Option<String>,
    pub eta: Option<String>,
    pub draught: Option<i64>,
    pub ship_length: Option<i64>,
    pub ship_width: Option<i64>,
    pub ship_type: Option<i64>,
    pub position_fixing_device_type: Option<i64>,
    pub report_class: Option<String>,
    pub sequence_number: Option<i64>,
    pub destination_mmsi: Option<i64>,
    pub safety_related_text: Option<String>,
}
