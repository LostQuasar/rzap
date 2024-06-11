/// Variable names and data format is converted from api documentation hosted at <https://api.openshock.app/swagger/index.html>
pub mod data_type {
    use serde::{Deserialize, Serialize};
    use strum_macros::EnumString;

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ListShockersResponse {
        pub shockers: Vec<ShockerResponse>,
        pub id: String,
        pub name: String,
        pub created_on: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ShockerResponse {
        pub name: Option<String>,
        pub is_paused: bool,
        pub created_on: String,
        pub id: String,
        pub rf_id: u32,
        pub model: ShockerModel,
    }

    #[derive(EnumString, Serialize, Deserialize, Debug)]
    pub enum ShockerModel {
        CaiXianlin,
        PetTrainer,
        Petrainer998DR,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct BaseResponse<T> {
        pub message: Option<String>,
        pub data: Option<T>,
    }

    #[derive(EnumString, Serialize, Deserialize, Debug)]
    pub enum ControlType {
        Stop,
        Shock,
        Vibrate,
        Sound,
    }
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ControlRequest {
        pub shocks: Vec<Shock>,
        pub custom_name: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Shock {
        pub id: String,
        #[serde(rename = "type")]
        pub control_type: ControlType,
        pub intensity: u8,
        pub duration: u16,
        pub exclusive: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct SelfResponse {
        pub id: String,
        pub name: Option<String>,
        pub email: Option<String>,
        pub image: Option<String>,
        pub rank: RankType,
    }

    #[derive(EnumString, Serialize, Deserialize, Debug)]
    pub enum RankType {
        User,
        Support,
        Staff,
        Admin,
        System,
    }
}
