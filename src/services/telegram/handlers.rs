use teloxide::prelude::*;
use crate::db::facade::DbFacade;
use super::keyboards::{registration_keyboard, schedule_keyboard, faculty_keyboard};
use std::sync::Arc;

/// Обработчик входящих сообщений от пользователя.
///
/// - `bot`: экземпляр Telegram‑бота.
/// - `msg`: сообщение от пользователя.
/// - `db`: фасад базы данных, обёрнутый в Arc для безопасного совместного использования.
///
/// Логика:
/// 1. Получаем Telegram ID отправителя.
/// 2. Проверяем, зарегистрирован ли студент в базе.
/// 3. Если зарегистрирован — приветствуем и показываем клавиатуру с расписанием.
/// 4. Если нет — предлагаем пройти регистрацию.
pub async fn handle_message(
    bot: Bot,
    msg: Message,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    // Извлекаем Telegram ID пользователя. Если поле отсутствует — используем 0.
    let telegram_id = msg.from.map(|u| u.id.0).unwrap_or(0);

    // Ищем студента в базе по Telegram ID.
    if let Some(student) = db.find_student(telegram_id.try_into().unwrap()).await.unwrap() {
        // Если студент найден — отправляем приветствие и клавиатуру для расписания.
        bot.send_message(msg.chat.id, format!("Привет, {}!", student.group_name))
            .reply_markup(schedule_keyboard())
            .await?;
    } else {
        // Если студент не найден — предлагаем пройти регистрацию.
        bot.send_message(msg.chat.id, "Ты ещё не зарегистрирован. Хочешь пройти регистрацию?")
            .reply_markup(registration_keyboard())
            .await?;
    }

    Ok(())
}

/// Обработчик callback‑запросов (нажатий на кнопки в интерфейсе).
///
/// - `bot`: экземпляр Telegram‑бота.
/// - `q`: объект callback‑запроса.
/// - `_db`: фасад базы данных (пока не используется, но оставлен для будущих расширений).
///
/// Логика:
/// 1. Проверяем, есть ли данные в callback.
/// 2. В зависимости от значения `data` отправляем разные сообщения и клавиатуры.
pub async fn handle_callback(
    bot: Bot,
    q: CallbackQuery,
    _db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    if let Some(data) = q.data {
        match data.as_str() {
            // Пользователь выбрал регистрацию — показываем клавиатуру факультетов.
            "register" => {
                bot.send_message(q.from.id, "Выбери факультет 📚")
                    .reply_markup(faculty_keyboard())
                    .await?;
            }
            // Пользователь запросил расписание.
            "my_schedule" => {
                bot.send_message(q.from.id, "Вот твоё расписание 📅").await?;
            }
            // Пользователь хочет выбрать группу.
            "choose_group" => {
                bot.send_message(q.from.id, "Выбери группу 🔎").await?;
            }
            // Любые другие значения игнорируем.
            _ => {}
        }
    }
    Ok(())
}
