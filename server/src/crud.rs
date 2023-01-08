use std::fmt::format;
use sqlx::sqlite::{SqlitePoolOptions, SqlitePool, SqliteQueryResult};
use std::str::FromStr;
use crate::models::*;
use md5;
use rand::prelude::*;
use sqlx::Row;
use sqlx::ColumnIndex;

async fn get_connection() -> tide::Result<SqlitePool> {
    let pool = SqlitePoolOptions::new().connect("sqlite:app.db").await?;
    Ok(pool)
}

pub async fn sqlx_create_group (data: &CreateGroupData) -> tide::Result<(String, u32)> {
    if !sqlx_is_user_real(data.user_id).await? {
        return Ok((("User does not exist.".to_string()), 0));
    }

    if !sqlx_is_logged(&data.token, data.user_id).await? {
        return Ok((("Wrong token.".to_string()), 0));
    }

    let connection = get_connection().await?;
    let tx = connection.begin().await.unwrap();

    let result = sqlx::query("INSERT INTO groups (is_closed) VALUES (0)").execute(&connection).await?;
    let insert = sqlx::query("INSERT INTO group_users (group_id, user_id, gift_recipient_id, is_admin) VALUES (?, ?, -1, 1)")
        .bind(result.last_insert_rowid())
        .bind(data.user_id)
        .execute(&connection).await?;

    tx.commit().await?;
    connection.close().await;

    return if insert.rows_affected() != 0 {
        Ok(("Success!".to_string(), result.last_insert_rowid().try_into().unwrap()))
    } else {
        Ok(("Wrong data".to_string(), 0))
    }
}

pub async fn sqlx_set_admin (data: &SetAdminData) -> tide::Result<(String)> {
    if !sqlx_is_user_real(data.user_id).await? {
        return Ok(("User does not exist.".to_string()));
    }

    if !sqlx_is_logged(&data.token, data.user_id).await? {
        return Ok(("Wrong token.".to_string()));
    }

    if !sqlx_is_group_real(data.group_id).await? {
        return Ok(("Group does not exist.".to_string()));
    }

    if !sqlx_is_in_group(data.user_id, data.group_id).await? {
        return Ok(("You are not in this group.".to_string()));
    }

    if !sqlx_is_user_real(data.new_admin_id).await? {
        return Ok(("New admin does not exist.".to_string()));
    }

    if !sqlx_is_in_group(data.new_admin_id, data.group_id).await? {
        return Ok(("New admin not in this group.".to_string()));
    }

    if !sqlx_is_admin_in_group(data.user_id, data.group_id).await? {
        return Ok(("You are not an admin.".to_string()));
    }

    if sqlx_is_admin_in_group(data.new_admin_id, data.group_id).await? {
        return Ok(("New admin is already an admin.".to_string()));
    }

    let connection = get_connection().await?;
    let tx = connection.begin().await.unwrap();

    let insert = sqlx::query("UPDATE group_users SET is_admin = true WHERE user_id=? AND group_id=?")
        .bind(data.new_admin_id)
        .bind(data.group_id)
        .execute(&connection).await?;

    tx.commit().await?;
    connection.close().await;

    return if insert.rows_affected() != 0 {
        Ok(("Success!".to_string()))
    } else {
        Ok(("Wrong data.".to_string()))
    }
}

pub async fn sqlx_leave_group (data: &LeaveGroupData) -> tide::Result<(String)> {
    if !sqlx_is_user_real(data.user_id).await? {
        return Ok(("User does not exist.".to_string()));
    }

    if !sqlx_is_logged(&data.token, data.user_id).await? {
        return Ok(("Wrong token.".to_string()));
    }

    if !sqlx_is_group_real(data.group_id).await? {
        return Ok(("Group does not exist.".to_string()));
    }

    if !sqlx_is_in_group(data.user_id, data.group_id).await? {
        return Ok(("You are already not in this group.".to_string()));
    }

    if sqlx_is_admin_in_group(data.user_id, data.group_id).await? {
        return Ok(("Admin can't leave its group.".to_string()));
    }

    let connection = get_connection().await?;
    let tx = connection.begin().await.unwrap();

    let insert = sqlx::query("DELETE FROM group_users WHERE user_id = ? AND group_id = ?")
        .bind(data.user_id)
        .bind(data.group_id)
        .execute(&connection).await?;

    tx.commit().await?;
    connection.close().await;

    return if insert.rows_affected() != 0 {
        Ok(("Success!".to_string()))
    } else {
        Ok(("Wrong data.".to_string()))
    }
}

pub async fn sqlx_stop_admin (data: &StopAdminData) -> tide::Result<(String)> {
    if !sqlx_is_user_real(data.user_id).await? {
        return Ok(("User does not exist.".to_string()));
    }

    if !sqlx_is_logged(&data.token, data.user_id).await? {
        return Ok(("Wrong token.".to_string()));
    }

    if !sqlx_is_group_real(data.group_id).await? {
        return Ok(("Group does not exist.".to_string()));
    }

    if !sqlx_is_in_group(data.user_id, data.group_id).await? {
        return Ok(("You are not in this group.".to_string()));
    }

    if !sqlx_is_admin_in_group(data.user_id, data.group_id).await? {
        return Ok(("You are not an admin.".to_string()));
    }

    if sqlx_is_admin_count_in_group(data.group_id).await? == 1 {
        return Ok(("You are the only admin.".to_string()));
    }

    let connection = get_connection().await?;
    let tx = connection.begin().await.unwrap();

    let insert = sqlx::query("UPDATE group_users SET is_admin = false WHERE user_id=? AND group_id=?")
        .bind(data.user_id)
        .bind(data.group_id)
        .execute(&connection).await?;

    tx.commit().await?;
    connection.close().await;

    return if insert.rows_affected() != 0 {
        Ok(("Success!".to_string()))
    } else {
        Ok(("Wrong data.".to_string()))
    }
}

pub async fn sqlx_join_group (data: &JoinGroupData) -> tide::Result<(String)> {
    if !sqlx_is_user_real(data.user_id).await? {
        return Ok(("User does not exist.".to_string()));
    }

    if !sqlx_is_logged(&data.token, data.user_id).await? {
        return Ok(("Wrong token.".to_string()));
    }

    if !sqlx_is_group_real(data.group_id).await? {
        return Ok(("Group does not exist.".to_string()));
    }

    if sqlx_is_in_group(data.user_id, data.group_id).await? {
        return Ok(("User is already in this group.".to_string()));
    }

    let connection = get_connection().await?;
    let tx = connection.begin().await.unwrap();

    let result = sqlx::query("INSERT INTO group_users (group_id, user_id, gift_recipient_id, is_admin) VALUES (?, ?, -1, 0)")
        .bind(data.group_id)
        .bind(data.user_id)
        .execute(&connection).await?;

    tx.commit().await?;
    connection.close().await;

    return if result.rows_affected() != 0 {
        Ok(("Success!".to_string()))
    } else {
        Ok(("Wrong data".to_string()))
    }
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
        sqlx::query("UPDATE users SET token = ?, is_logged = true WHERE user_id=?")
            .bind(&token)
            .bind(data.user_id)
            .execute(&connection).await?;
    } else {
        return Ok(("".to_string(), "Wrong password.".to_string()));
    }

    tx.commit().await?;
    connection.close().await;

    return Ok((token, "Success!".to_string()))
}

async fn sqlx_is_logged(token: &str, user_id: u32) -> tide::Result<bool> {
    let connection = get_connection().await?;
    let tx = connection.begin().await.unwrap();

    let result = sqlx::query("SELECT * FROM users WHERE user_id = ? AND token = ? AND is_logged = true")
        .bind(user_id)
        .bind(&token)
        .fetch_all(&connection).await?;

    tx.commit().await?;
    connection.close().await;

    return if result.is_empty() {
        Ok(false)
    } else {
        Ok(true)
    }
}

async fn sqlx_is_in_group(user_id: u32, group_id: u32) -> tide::Result<bool> {
    let connection = get_connection().await?;
    let tx = connection.begin().await.unwrap();

    let result = sqlx::query("SELECT * FROM group_users WHERE user_id = ? AND group_id = ?")
        .bind(user_id)
        .bind(group_id)
        .fetch_all(&connection).await?;

    tx.commit().await?;
    connection.close().await;

    return if result.is_empty() {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub async fn sqlx_logoff (data: &LogoffData) -> tide::Result<String> {
    if !sqlx_is_logged(&data.token, data.user_id).await? {
        return Ok("Wrong password.".to_string());
    }

    let connection = get_connection().await?;
    let tx = connection.begin().await.unwrap();

    sqlx::query("UPDATE users SET is_logged = false WHERE user_id = ?")
        .bind(data.user_id)
        .execute(&connection).await?;

    tx.commit().await?;
    connection.close().await;

    return Ok("Success!".to_string())
}

async fn sqlx_is_admin_in_group(user_id: u32, group_id: u32) -> tide::Result<bool> {
    let connection = get_connection().await?;
    let tx = connection.begin().await.unwrap();

    let result = sqlx::query("SELECT is_admin FROM group_users WHERE user_id = ? AND group_id = ?")
        .bind(user_id)
        .bind(group_id)
        .fetch_all(&connection).await?;

    tx.commit().await?;
    connection.close().await;

    return if result.is_empty() {
        Ok(false)
    } else {
        return if result[0].get::<u32, &str>("is_admin") == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

async fn sqlx_is_user_real(user_id: u32) -> tide::Result<bool> {
    let connection = get_connection().await?;
    let tx = connection.begin().await.unwrap();

    let result = sqlx::query("SELECT * FROM users WHERE user_id = ?")
        .bind(user_id)
        .fetch_all(&connection).await?;

    tx.commit().await?;
    connection.close().await;

    return if result.is_empty() {
        Ok(false)
    } else {
        Ok(true)
    }
}

async fn sqlx_is_group_real(group_id: u32) -> tide::Result<bool> {
    let connection = get_connection().await?;
    let tx = connection.begin().await.unwrap();

    let result = sqlx::query("SELECT * FROM groups WHERE group_id = ?")
        .bind(group_id)
        .fetch_all(&connection).await?;

    tx.commit().await?;
    connection.close().await;

    return if result.is_empty() {
        Ok(false)
    } else {
        Ok(true)
    }
}

async fn sqlx_is_admin_count_in_group(group_id: u32) -> tide::Result<usize> {
    let connection = get_connection().await?;
    let tx = connection.begin().await.unwrap();

    let result = sqlx::query("SELECT * FROM group_users WHERE group_id = ? and is_admin = true")
        .bind(group_id)
        .fetch_all(&connection).await?;

    tx.commit().await?;
    connection.close().await;

    Ok(result.len())
}