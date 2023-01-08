use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct CreateGroupData {
    pub user_id: u32,
    pub token: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateGroupResp {
    pub group_id: u32
}

#[derive(Debug, Deserialize)]
pub struct JoinGroupData {
    pub user_id: u32,
    pub group_id: u32,
    pub token: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JoinGroupResp {
    pub status: String
}

#[derive(Debug, Deserialize)]
pub struct SignupData {
    pub name: String,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupResp {
    pub token: String,
    pub user_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct LoginData {
    pub user_id: u32,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorStatus {
    pub reason: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResp {
    pub token: String
}

#[derive(Debug, Deserialize)]
pub struct LogoffData {
    pub token: String,
    pub user_id: u32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogoffResp {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct SetAdminData {
    pub token: String,
    pub user_id: u32,
    pub group_id: u32,
    pub new_admin_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SetAdminResp {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct StopAdminData {
    pub token: String,
    pub user_id: u32,
    pub group_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StopAdminResp {
    pub status: String,
}