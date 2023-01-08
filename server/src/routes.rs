use crate::models;
use crate::crud;
use tide::prelude::*;
use serde_json::{Result, Value};
use crate::crud::*;
use crate::models::{ErrorStatus, LoginResp, SignupResp};

async fn get_json_params(request: &mut tide::Request<()>) -> tide::Result<serde_json::Value> {
    let body_str = request.body_string().await.unwrap();
    Ok(serde_json::from_str(body_str.as_str())?)
}
// переделать под deserialize macro
pub async fn create_user(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let json: serde_json::Value = get_json_params(&mut request).await.unwrap();

    let user = serde_json::from_value::<models::CreateUser>(json)?;
    let result = sqlx_create_user(&user).await?;
    Ok(tide::Response::builder(201)
        .body(String::from("{\n\t\"name\": \"") +
              &user.name +
              &String::from("\",\n\t\"user_id\":") +
              &result.last_insert_rowid().to_string() +
              &String::from("\n}")
              )
        .build()
    )
}

pub async fn create_group(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let json: serde_json::Value = get_json_params(&mut request).await.unwrap();

    let group = serde_json::from_value::<models::CreateGroup>(json)?;
    let result = sqlx_create_group(&group).await?;
    Ok(tide::Response::builder(201)
        .body(String::from("{\n\t\"user_id\": ") +
            &group.user_id.to_string() +
            &String::from(",\n\t\"group_id\": ") +
            &result.last_insert_rowid().to_string() +
            &String::from("\n}")
        )
        .build()
    )
}




pub async fn signup(mut request: tide::Request<()>) -> tide::Result<tide::Response> {
    let json: serde_json::Value = get_json_params(&mut request).await.unwrap();

    let data = serde_json::from_value::<models::SignupData>(json)?;


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
    let json: serde_json::Value = get_json_params(&mut request).await.unwrap();

    let data = serde_json::from_value::<models::LoginData>(json)?;


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