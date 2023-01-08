use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateGroup {
    pub user_id: u32,
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