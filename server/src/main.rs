mod crud;
mod routes;
mod models;

use std::env;
use tide::prelude::*;
use tide::utils::async_trait;
use crate::routes::{create_group, create_user, signup, login};
use tide::log;

#[async_std::main]
async fn main () -> tide::Result<()> {
    log::start();

    let mut app = tide::new();

    app.at("/create_user").post(create_user); // уникальное имя, вернуть структуру пользователя
    app.at("/create_group").post(create_group); // вернуть список участников, id группы
    app.at("/signup").post(signup); // вернуть список участников, id группы
    app.at("/login").post(login); // вернуть список участников, id группы
    //app.at("/join_group").post(()); // вернуть список участников, id группы

    //app.at("/set_admin").post(()); // вернуть статус (пользователь админ)
    //app.at("/stop_admin").post(()); // вернуть статус (не меньше одного админа)
    //app.at("/leave_group").post(()); // вернуть статус (участник не админ, группа не закрыта или есть ещё хотя бы один админ)
    //app.at("/delete_group").delete(()); // вернуть статус (участник админ)

    //app.at("/christmas").post(()); // вернуть статус (жеребьевка, запускает админ, группа закрывает)

    //app.at("/get_group").get(()); // список участников группы, id группы (REST)
    //app.at("/get_recipient").get(()); // информацию о получателе подарков (REST)

    app.listen("127.0.0.1:80").await?;

    Ok(())
}