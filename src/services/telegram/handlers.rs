use teloxide::prelude::*;
use crate::db::facade::DbFacade;
use crate::domain::buttons::TechButton;
use crate::domain::faculty::Faculty;
use super::keyboards::{registration_keyboard, schedule_keyboard, faculty_keyboard};
use std::sync::Arc;
use log::{info, debug, warn, error};
use std::str::FromStr;

/// Обработчик входящих сообщений от пользователя.
///
/// Архитектурное решение: мы сразу разделяем логику на два сценария —
/// зарегистрированный студент и новый пользователь. Это позволяет
/// держать код читаемым и расширяемым.
///
/// - `bot`: экземпляр Telegram‑бота.
/// - `msg`: сообщение от пользователя.
/// - `db`: фасад базы данных, обёрнутый в Arc для безопасного доступа из разных потоков.
pub async fn handle_message(
    bot: Bot,
    msg: Message,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    // Telegram ID — ключ для идентификации пользователя.
    let telegram_id = msg.from.map(|u| u.id.0).unwrap_or(0);
    debug!("handle_message: получен запрос от пользователя с telegram_id={}", telegram_id);

    // Ищем студента в базе по Telegram ID.
    match db.find_student(telegram_id.try_into().unwrap()).await {
        Ok(Some(student)) => {
            // ✅ Сценарий: студент найден
            info!("Пользователь {} найден в базе, группа: {}", telegram_id, student.group_name);
            bot.send_message(msg.chat.id, format!("Привет, {}!", student.group_name))
                .reply_markup(schedule_keyboard())
                .await?;
        }
        Ok(None) => {
            // ⚠️ Сценарий: студент не найден
            warn!("Пользователь {} не найден в базе, предлагаем регистрацию", telegram_id);
            bot.send_message(msg.chat.id, "Ты ещё не зарегистрирован. Хочешь пройти регистрацию?")
                .reply_markup(registration_keyboard())
                .await?;
        }
        Err(err) => {
            // ❌ Сценарий: ошибка базы данных
            error!("Ошибка при поиске пользователя {} в базе: {:?}", telegram_id, err);
            bot.send_message(msg.chat.id, "Произошла ошибка при обращении к базе данных ❌")
                .await?;
        }
    }

    Ok(())
}

/// Обработчик callback‑запросов (нажатий на кнопки).
///
/// Архитектурное решение: мы используем строгую типизацию через `enum`
/// (`TechButton`, `Faculty`), чтобы не работать со «старыми» строками.
/// Это снижает риск ошибок и делает код самодокументируемым.
///
/// - `bot`: экземпляр Telegram‑бота.
/// - `q`: объект callback‑запроса.
/// - `db`: фасад базы данных (пока не используется, но оставлен для будущих расширений).
pub async fn handle_callback(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    if let Some(data) = q.data {
        debug!(
            "handle_callback: получен callback от пользователя {} с данными '{}'",
            q.from.id, data
        );

        // 1️⃣ Сначала пробуем распарсить как TechButton (технические кнопки)
        if let Ok(btn) = TechButton::from_str(&data) {
            match btn {
                TechButton::Register => {
                    info!("Пользователь {} выбрал регистрацию", q.from.id);
                    bot.send_message(q.from.id, "Выбери факультет 📚")
                        .reply_markup(faculty_keyboard())
                        .await?;
                }
                TechButton::MySchedule => {
                    info!("Пользователь {} запросил расписание", q.from.id);
                    bot.send_message(q.from.id, "Вот твоё расписание 📅").await?;
                }
                TechButton::ChooseGroup => {
                    info!("Пользователь {} хочет выбрать группу", q.from.id);
                    bot.send_message(q.from.id, "Выбери группу 🔎").await?;
                }
            }
            return Ok(());
        }

        // 2️⃣ Если не TechButton — пробуем как Faculty
        if let Ok(fac) = Faculty::from_str(&data) {
            info!("Пользователь {} выбрал факультет {:?}", q.from.id, fac);
            bot.send_message(q.from.id, format!("Ты выбрал: {}", fac.title()))
                .await?;
            return Ok(());
        }

        // 3️⃣ Если ничего не подошло — логируем как неизвестный callback
        warn!(
            "Получен неизвестный callback '{}' от пользователя {}",
            data, q.from.id
        );
    } else {
        warn!("handle_callback: пустой callback от пользователя {}", q.from.id);
    }
    Ok(())
}
