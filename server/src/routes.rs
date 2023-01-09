#[allow(unused_variables)]

use crate::models;
use crate::crud;
use tide::prelude::*;
use serde_json::{Result, Value};
use crate::crud::*;
use crate::models::{ErrorStatus, LoginResp, SignupResp, LogoffResp, CreateGroupResp, GetUserNameByIdResp, GetUserNameByIdData};
use crate::models::{SetAdminResp, LeaveGroupResp, StopAdminResp, JoinGroupResp,
                    DeleteGroupResp, ChristmasResp, GetGiftRecipientIdResp};

async fn get_json_params(request: &mut tide::Request<()>) -> tide::Result<serde_json::Value> {
    let body_str = request.body_string().await.unwrap();
    Ok(serde_json::from_str(body_str.as_str())?)
}

pub async fn get_gift_recipient_id(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let json = match json {
        Ok(json) => json,
        Err(json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let data = serde_json::from_value::<models::GetGiftRecipientIdData>(json);

    let data = match data {
        Ok(data) => data,
        Err(data) => return Ok (
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };


    let (status, gift_recipient_id) = sqlx_get_gift_recipient_id(&data).await?;
    return if status == "Success!" {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<GetGiftRecipientIdResp>(&GetGiftRecipientIdResp {gift_recipient_id})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(422)
            .body(
                serde_json::to_string::<ErrorStatus>(&ErrorStatus {reason: status})?
            )
            .build()
        )
    }
}

pub async fn create_group(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let json = match json {
        Ok(json) => json,
        Err(json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let data = serde_json::from_value::<models::CreateGroupData>(json);

    let data = match data {
        Ok(data) => data,
        Err(data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };


    let (status, result) = sqlx_create_group(&data).await?;
    return if status == "Success!" {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<CreateGroupResp>(&CreateGroupResp {group_id: result })?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(422)
            .body(
                serde_json::to_string::<ErrorStatus>(&ErrorStatus {reason: status })?
            )
            .build()
        )
    }
}

pub async fn join_group(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    //println!("join group endpoint.");
    let json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let json = match json {
        Ok(json) => json,
        Err(json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let data = serde_json::from_value::<models::JoinGroupData>(json);

    let data = match data {
        Ok(data) => data,
        Err(data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let (status) = sqlx_join_group(&data).await?;

    return if status == "Success!" {
        Ok (tide::Response::builder(201)
            .body(
                serde_json::to_string::<JoinGroupResp>(&JoinGroupResp {status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(422)
            .body(
                serde_json::to_string::<ErrorStatus>(&ErrorStatus {reason: status })?
            )
            .build()
        )
    }
}


pub async fn signup(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let json = match json {
        Ok(json) => json,
        Err(json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let data = serde_json::from_value::<models::SignupData>(json);

    let data = match data {
        Ok(data) => data,
        Err(data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let (token, status, user_id) = sqlx_signup(&data).await?;

    return if token.len() == 0 {
        Ok(tide::Response::builder(422)
            .body(
                serde_json::to_string::<ErrorStatus>(&ErrorStatus {reason: status })?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<SignupResp>(&SignupResp { token, user_id })?
            )
            .build()
        )
    }
}

pub async fn login(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let json = match json {
        Ok(json) => json,
        Err(json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let data = serde_json::from_value::<models::LoginData>(json);

    let data = match data {
        Ok(data) => data,
        Err(data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let (token, status) = sqlx_login(&data).await?;

    return if token.len() == 0 {
        Ok(tide::Response::builder(403)
            .body(
                serde_json::to_string::<ErrorStatus>(&ErrorStatus{reason: status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<LoginResp>(&LoginResp{token})?
            )
            .build()
        )
    }
}


pub async fn logoff(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let json = match json {
        Ok(json) => json,
        Err(json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let data = serde_json::from_value::<models::LogoffData>(json);

    let data = match data {
        Ok(data) => data,
        Err(data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };


    let (status) = sqlx_logoff(&data).await?;

    return if status != "Success!" {
        Ok(tide::Response::builder(403)
            .body(
                serde_json::to_string::<ErrorStatus>(&ErrorStatus{reason: status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<LogoffResp>(&LogoffResp{status})?
            )
            .build()
        )
    }
}

pub async fn set_admin(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let json = match json {
        Ok(json) => json,
        Err(json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let data = serde_json::from_value::<models::SetAdminData>(json);

    let data = match data {
        Ok(data) => data,
        Err(data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let (status) = sqlx_set_admin(&data).await?;

    return if status != "Success!" {
        Ok(tide::Response::builder(403)
            .body(
                serde_json::to_string::<ErrorStatus>(&ErrorStatus{reason: status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<SetAdminResp>(&SetAdminResp{status})?
            )
            .build()
        )
    }
}

pub async fn stop_admin(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let json = match json {
        Ok(json) => json,
        Err(json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let data = serde_json::from_value::<models::StopAdminData>(json);

    let data = match data {
        Ok(data) => data,
        Err(data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let (status) = sqlx_stop_admin(&data).await?;

    return if status != "Success!" {
        Ok(tide::Response::builder(403)
            .body(
                serde_json::to_string::<ErrorStatus>(&ErrorStatus{reason: status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<StopAdminResp>(&StopAdminResp{status})?
            )
            .build()
        )
    }
}

pub async fn leave_group(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let json = match json {
        Ok(json) => json,
        Err(json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let data = serde_json::from_value::<models::LeaveGroupData>(json);

    let data = match data {
        Ok(data) => data,
        Err(data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let (status) = sqlx_leave_group(&data).await?;

    return if status != "Success!" {
        Ok(tide::Response::builder(403)
            .body(
                serde_json::to_string::<ErrorStatus>(&ErrorStatus{reason: status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<LeaveGroupResp>(&LeaveGroupResp{status})?
            )
            .build()
        )
    }
}

pub async fn delete_group(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let json = match json {
        Ok(json) => json,
        Err(json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let data = serde_json::from_value::<models::DeleteGroupData>(json);

    let data = match data {
        Ok(data) => data,
        Err(data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let (status) = sqlx_delete_group(&data).await?;

    return if status != "Success!" {
        Ok(tide::Response::builder(403)
            .body(
                serde_json::to_string::<ErrorStatus>(&ErrorStatus{reason: status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<DeleteGroupResp>(&DeleteGroupResp{status})?
            )
            .build()
        )
    }
}

pub async fn christmas(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let json = match json {
        Ok(json) => json,
        Err(json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let data = serde_json::from_value::<models::ChristmasData>(json);

    let data = match data {
        Ok(data) => data,
        Err(data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let (status) = sqlx_christmas(&data).await?;

    return if status != "Success!" {
        Ok(tide::Response::builder(403)
            .body(
                serde_json::to_string::<ErrorStatus>(&ErrorStatus{reason: status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<ChristmasResp>(&ChristmasResp{status})?
            )
            .build()
        )
    }
}

pub async fn get_user_name_by_id(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let json = match json {
        Ok(json) => json,
        Err(json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let data = serde_json::from_value::<models::GetUserNameByIdData>(json);

    let data = match data {
        Ok(data) => data,
        Err(data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let (status, name) = sqlx_user_name_by_id(&data).await?;
    println!("{}", name);
    return if status != "Success!" {
        Ok(tide::Response::builder(403)
            .body(
                serde_json::to_string::<ErrorStatus>(&ErrorStatus{ reason: status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<GetUserNameByIdResp>(&GetUserNameByIdResp{name})?
            )
            .build()
        )
    }
}