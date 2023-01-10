use crate::models;
use crate::crud;
use serde_json;

async fn get_json_params(request: &mut tide::Request<()>) -> tide::Result<serde_json::Value> {
    let body_str = request.body_string().await.unwrap();
    Ok(serde_json::from_str(body_str.as_str())?)
}

pub async fn get_gift_recipient_id(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let _json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let _json = match _json {
        Ok(_json) => _json,
        Err(_json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let _data = serde_json::from_value::<models::GetGiftRecipientIdData>(_json);

    let _data = match _data {
        Ok(_data) => _data,
        Err(_data) => return Ok (
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };


    let (status, gift_recipient_id) = crud::sqlx_get_gift_recipient_id(&_data).await?;
    return if status == "Success!" {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<models::GetGiftRecipientIdResp>(&models::GetGiftRecipientIdResp {gift_recipient_id})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(422)
            .body(
                serde_json::to_string::<models::ErrorStatus>(&models::ErrorStatus {reason:status})?
            )
            .build()
        )
    }
}

pub async fn create_group(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let _json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let _json = match _json {
        Ok(_json) => _json,
        Err(_json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let _data = serde_json::from_value::<models::CreateGroupData>(_json);

    let _data = match _data {
        Ok(_data) => _data,
        Err(_data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };


    let (status, result) = crud::sqlx_create_group(&_data).await?;
    return if status == "Success!" {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<models::CreateGroupResp>(&models::CreateGroupResp {group_id: result })?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(422)
            .body(
                serde_json::to_string::<models::ErrorStatus>(&models::ErrorStatus {reason: status })?
            )
            .build()
        )
    }
}

pub async fn join_group(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    //println!("join group endpoint.");
    let _json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let _json = match _json {
        Ok(_json) => _json,
        Err(_json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let _data = serde_json::from_value::<models::JoinGroupData>(_json);

    let _data = match _data {
        Ok(_data) => _data,
        Err(_data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let status = crud::sqlx_join_group(&_data).await?;

    return if status == "Success!" {
        Ok (tide::Response::builder(201)
            .body(
                serde_json::to_string::<models::JoinGroupResp>(&models::JoinGroupResp {status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(422)
            .body(
                serde_json::to_string::<models::ErrorStatus>(&models::ErrorStatus {reason: status })?
            )
            .build()
        )
    }
}


pub async fn signup(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let _json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let _json = match _json {
        Ok(_json) => _json,
        Err(_json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let _data = serde_json::from_value::<models::SignupData>(_json);

    let _data = match _data {
        Ok(_data) => _data,
        Err(_data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let (token, status, user_id) = crud::sqlx_signup(&_data).await?;

    return if token.len() == 0 {
        Ok(tide::Response::builder(422)
            .body(
                serde_json::to_string::<models::ErrorStatus>(&models::ErrorStatus {reason: status })?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<models::SignupResp>(&models::SignupResp { token, user_id })?
            )
            .build()
        )
    }
}

pub async fn login(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let _json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let _json = match _json {
        Ok(_json) => _json,
        Err(_json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let _data = serde_json::from_value::<models::LoginData>(_json);

    let _data = match _data {
        Ok(_data) => _data,
        Err(_data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let (token, status) = crud::sqlx_login(&_data).await?;

    return if token.len() == 0 {
        Ok(tide::Response::builder(403)
            .body(
                serde_json::to_string::<models::ErrorStatus>(&models::ErrorStatus{reason: status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<models::LoginResp>(&models::LoginResp{token})?
            )
            .build()
        )
    }
}


pub async fn logoff(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let _json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let _json = match _json {
        Ok(_json) => _json,
        Err(_json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let _data = serde_json::from_value::<models::LogoffData>(_json);

    let _data = match _data {
        Ok(_data) => _data,
        Err(_data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };


    let status = crud::sqlx_logoff(&_data).await?;

    return if status != "Success!" {
        Ok(tide::Response::builder(403)
            .body(
                serde_json::to_string::<models::ErrorStatus>(&models::ErrorStatus{reason: status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<models::LogoffResp>(&models::LogoffResp{status})?
            )
            .build()
        )
    }
}

pub async fn set_admin(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let _json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let _json = match _json {
        Ok(_json) => _json,
        Err(_json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let _data = serde_json::from_value::<models::SetAdminData>(_json);

    let _data = match _data {
        Ok(_data) => _data,
        Err(_data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let status = crud::sqlx_set_admin(&_data).await?;

    return if status != "Success!" {
        Ok(tide::Response::builder(403)
            .body(
                serde_json::to_string::<models::ErrorStatus>(&models::ErrorStatus{reason: status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<models::SetAdminResp>(&models::SetAdminResp{status})?
            )
            .build()
        )
    }
}

pub async fn stop_admin(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let _json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let _json = match _json {
        Ok(_json) => _json,
        Err(_json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let _data = serde_json::from_value::<models::StopAdminData>(_json);

    let _data = match _data {
        Ok(_data) => _data,
        Err(_data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let status = crud::sqlx_stop_admin(&_data).await?;

    return if status != "Success!" {
        Ok(tide::Response::builder(403)
            .body(
                serde_json::to_string::<models::ErrorStatus>(&models::ErrorStatus{reason: status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<models::StopAdminResp>(&models::StopAdminResp{status})?
            )
            .build()
        )
    }
}

pub async fn leave_group(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let _json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let _json = match _json {
        Ok(_json) => _json,
        Err(_json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let _data = serde_json::from_value::<models::LeaveGroupData>(_json);

    let _data = match _data {
        Ok(_data) => _data,
        Err(_data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let status = crud::sqlx_leave_group(&_data).await?;

    return if status != "Success!" {
        Ok(tide::Response::builder(403)
            .body(
                serde_json::to_string::<models::ErrorStatus>(&models::ErrorStatus{reason: status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<models::LeaveGroupResp>(&models::LeaveGroupResp{status})?
            )
            .build()
        )
    }
}

pub async fn delete_group(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let _json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let _json = match _json {
        Ok(_json) => _json,
        Err(_json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let _data = serde_json::from_value::<models::DeleteGroupData>(_json);

    let _data = match _data {
        Ok(_data) => _data,
        Err(_data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let status = crud::sqlx_delete_group(&_data).await?;

    return if status != "Success!" {
        Ok(tide::Response::builder(403)
            .body(
                serde_json::to_string::<models::ErrorStatus>(&models::ErrorStatus{reason: status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<models::DeleteGroupResp>(&models::DeleteGroupResp{status})?
            )
            .build()
        )
    }
}

pub async fn christmas(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let _json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let _json = match _json {
        Ok(_json) => _json,
        Err(_json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let _data = serde_json::from_value::<models::ChristmasData>(_json);

    let _data = match _data {
        Ok(_data) => _data,
        Err(_data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let status = crud::sqlx_christmas(&_data).await?;

    return if status != "Success!" {
        Ok(tide::Response::builder(403)
            .body(
                serde_json::to_string::<models::ErrorStatus>(&models::ErrorStatus{reason: status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<models::ChristmasResp>(&models::ChristmasResp{status})?
            )
            .build()
        )
    }
}

pub async fn get_user_name_by_id(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let _json: tide::Result<serde_json::Value> = get_json_params(&mut request).await;

    let _json = match _json {
        Ok(_json) => _json,
        Err(_json) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let _data = serde_json::from_value::<models::GetUserNameByIdData>(_json);

    let _data = match _data {
        Ok(_data) => _data,
        Err(_data) => return Ok(
            tide::Response::builder(422)
                .body("{\"reason\": \"Wrong syntax\"}")
                .build()
        )
    };

    let (status, name) = crud::sqlx_user_name_by_id(&_data).await?;
    println!("{}", name);
    return if status != "Success!" {
        Ok(tide::Response::builder(403)
            .body(
                serde_json::to_string::<models::ErrorStatus>(&models::ErrorStatus{ reason: status})?
            )
            .build()
        )
    } else {
        Ok(tide::Response::builder(201)
            .body(
                serde_json::to_string::<models::GetUserNameByIdResp>(&models::GetUserNameByIdResp{name})?
            )
            .build()
        )
    }
}