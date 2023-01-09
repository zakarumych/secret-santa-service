mod crud;
mod routes;
mod models;

use std::env;
use std::fmt::format;
use dotenvy::dotenv;
use tide::prelude::*;
use tide::utils::async_trait;
use crate::routes::{create_group, join_group, signup, login, logoff,
                    set_admin, stop_admin, leave_group, delete_group, christmas,
                    get_gift_recipient_id, get_user_name_by_id};
use tide::log;

#[async_std::main]
async fn main () -> tide::Result<()> {
    log::start();
    dotenv().ok();

    let mut app = tide::new();

    app.at("/create_group").post(create_group); // вернуть id группы

    app.at("/signup").post(signup); // регистрация участника, set is_logged=true, вернуть токен
    app.at("/login").post(login); // выполнить вход, перезаписывает токен, если is_logged=true
    app.at("/logoff").post(logoff); // выход из системы, обнуляет is_logged
    app.at("/join_group").post(join_group); // вернуть статус

    app.at("/set_admin").post(set_admin); // вернуть статус (пользователь админ)
    app.at("/stop_admin").post(stop_admin); // вернуть статус (не меньше одного админа)
    app.at("/leave_group").post(leave_group); // вернуть статус (участник не админ, группа не закрыта или есть ещё хотя бы один админ)
    app.at("/delete_group").post(delete_group); // вернуть статус (участник админ)
    app.at("/christmas").post(christmas); // вернуть статус (жеребьевка, запускает админ, группа закрывается)

    app.at("/get_gift_recipient_id").post(get_gift_recipient_id); // список участников группы, id группы (REST)
    app.at("/get_user_name_by_id").post(get_user_name_by_id);
    app.listen(format!("{}:{}", std::env::var("ADDRESS").expect("ADDRESS must be set."),
                                       std::env::var("PORT").expect("PORT"))).await?;

    Ok(())
}