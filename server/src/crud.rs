use std::fmt::format;
use sqlx::sqlite::{SqlitePoolOptions, SqlitePool, SqliteQueryResult};
use std::str::FromStr;
use crate::models::*;
use md5;
use rand::prelude::*;
use sqlx::Row;

pub async fn get_connection() -> tide::Result<SqlitePool> {
    let pool = SqlitePoolOptions::new().connect("sqlite:app.db").await?;
    Ok(pool)
}

pub async fn sqlx_create_user (user: &CreateUser) -> tide::Result<SqliteQueryResult> {
    let connection = get_connection().await?;
    let tx = connection.begin().await.unwrap();
    let result = sqlx::query("INSERT INTO users (name) VALUES (?)").bind(&user.name).execute(&connection).await?;
    tx.commit().await?;
    connection.close().await;
    Ok(result.into())
}

pub async fn sqlx_create_group (group: &CreateGroup) -> tide::Result<SqliteQueryResult> {
    let connection = get_connection().await?;
    let tx = connection.begin().await.unwrap();
    let result = sqlx::query("INSERT INTO groups (is_closed) VALUES (0)").execute(&connection).await?;
    sqlx::query("INSERT INTO group_users (group_id, user_id, gift_recipient_id, is_admin) VALUES (?, ?, -1, 1)")
        .bind(result.last_insert_rowid())
        .bind(group.user_id)
        .execute(&connection).await?;

    tx.commit().await?;
    connection.close().await;
    Ok(result.into())
}

pub async fn sqlx_signup (data: &SignupData) -> tide::Result<(String, String, u32)> {
    if data.password.len() < 8 {
        return Ok(("Password is too short.".to_string(), "".to_string(), 0))
    }

    if data.name.len() < 5 {
        return Ok(("Username is too short.".to_string(), "".to_string(), 0))
    }

    if !data.password.is_ascii() {
        return Ok(("Password is not ascii-only.".to_string(), "".to_string(), 0))
    }

    if !data.name.is_ascii() {
        return Ok(("Username is not ascii-only.".to_string(), "".to_string(), 0))
    }

    let hash_psw = format!("{:x}", md5::compute::<&str>(&data.password));
    let token = format!("{:x}", md5::compute::<[u8;16]>(rand::random::<[u8;16]>()));
    let connection = get_connection().await?;
    let tx = connection.begin().await.unwrap();

    let result = sqlx::query("INSERT OR IGNORE INTO users (name, token, hash_psw, is_logged) VALUES (?, ?, ?, true)")
        .bind(&data.name)
        .bind(&token)
        .bind(hash_psw)
        .execute(&connection).await?;

    tx.commit().await?;
    connection.close().await;

    return Ok((token.clone(), "Success!".to_string(), result.last_insert_rowid().try_into().unwrap()))
}

pub async fn sqlx_login (data: &LoginData) -> tide::Result<(String, String)> {
    let hash_psw = format!("{:x}", md5::compute::<&str>(&data.password));
    let connection = get_connection().await?;
    let mut token = String::new();
    let tx = connection.begin().await.unwrap();

    let result = sqlx::query("SELECT * FROM users WHERE user_id = ? AND hash_psw = ?")
        .bind(&data.user_id)
        .bind(&hash_psw)
        .fetch_one(&connection).await?;

    if !result.is_empty() {
        token = format!("{:x}", md5::compute::<[u8;16]>(rand::random::<[u8;16]>()));
        sqlx::query("UPDATE users SET token = ? WHERE user_id=?")
            .bind(&token)
            .bind(&data.user_id)
            .execute(&connection).await?;
    } else {
        return Ok(("".to_string(), "Wrong password".to_string()));
    }

    tx.commit().await?;
    connection.close().await;

    return Ok((token, "Success!".to_string()))
}