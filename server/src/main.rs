mod crud;
mod routes;
mod models;

use dotenvy::dotenv;
use tide;

#[async_std::main]
async fn main () -> tide::Result<()> {
    tide::log::start();
    dotenv().ok();

    let mut app = tide::new();

    app.at("/create_group").post(routes::create_group); // вернуть id группы

    app.at("/signup").post(routes::signup); // регистрация участника, set is_logged=true, вернуть токен
    app.at("/login").post(routes::login); // выполнить вход, перезаписывает токен, если is_logged=true
    app.at("/logoff").post(routes::logoff); // выход из системы, обнуляет is_logged
    app.at("/join_group").post(routes::join_group); // вернуть статус

    app.at("/set_admin").post(routes::set_admin); // вернуть статус (пользователь админ)
    app.at("/stop_admin").post(routes::stop_admin); // вернуть статус (не меньше одного админа)
    app.at("/leave_group").post(routes::leave_group); // вернуть статус (участник не админ, группа не закрыта или есть ещё хотя бы один админ)
    app.at("/delete_group").post(routes::delete_group); // вернуть статус (участник админ)
    app.at("/christmas").post(routes::christmas); // вернуть статус (жеребьевка, запускает админ, группа закрывается)

    app.at("/get_gift_recipient_id").post(routes::get_gift_recipient_id); // список участников группы, id группы (REST)
    app.at("/get_user_name_by_id").post(routes::get_user_name_by_id);
    app.listen(format!("{}:{}", std::env::var("ADDRESS").expect("ADDRESS must be set."),
                                       std::env::var("PORT").expect("PORT"))).await?;

    Ok(())
}