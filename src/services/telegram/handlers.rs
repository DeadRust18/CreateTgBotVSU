use teloxide::prelude::*;
use crate::db::facade::DbFacade;
use crate::domain::buttons::TechButton;
use crate::domain::faculty::Faculty;
use crate::domain::study_form::StudyForm;
use crate::domain::course::Course;
use crate::domain::groups::mit::MitGroup;
use crate::services::telegram::keyboards::{
    registration_keyboard, schedule_keyboard, faculty_keyboard,
    study_form_keyboard, course_keyboard, mit_group_keyboard,
};
use std::sync::Arc;
use log::{info, debug, warn, error};
use std::str::FromStr;

/// Обработчик входящих сообщений от пользователя.
/// Здесь мы проверяем, зарегистрирован ли студент, и показываем ему меню.
pub async fn handle_message(
    bot: Bot,
    msg: Message,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    let telegram_id = msg.from.map(|u| u.id.0).unwrap_or(0);
    debug!("handle_message: получен запрос от пользователя с telegram_id={}", telegram_id);

    match db.find_student(telegram_id.try_into().unwrap()).await {
        Ok(Some(student)) => {
            info!("Пользователь {} найден в базе, группа: {}, форма: {}", telegram_id, student.group_name, student.study_form);
            bot.send_message(msg.chat.id, format!("Привет, {}!", student.group_name))
                .reply_markup(schedule_keyboard())
                .await?;
        }
        Ok(None) => {
            warn!("Пользователь {} не найден в базе, предлагаем регистрацию", telegram_id);
            bot.send_message(msg.chat.id, "Ты ещё не зарегистрирован. Хочешь пройти регистрацию?")
                .reply_markup(registration_keyboard())
                .await?;
        }
        Err(err) => {
            error!("Ошибка при поиске пользователя {} в базе: {:?}", telegram_id, err);
            bot.send_message(msg.chat.id, "Произошла ошибка при обращении к базе данных ❌")
                .await?;
        }
    }

    Ok(())
}

/// Обработчик callback-запросов (нажатий на кнопки).
/// Здесь мы пошагово разбираем выбор пользователя: факультет → форма → курс → группа.
pub async fn handle_callback(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    if let Some(data) = q.data {
        debug!("handle_callback: получен callback от пользователя {} с данными '{}'", q.from.id, data);

        // 1️⃣ Технические кнопки (регистрация, расписание, выбор группы)
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
                    bot.send_message(q.from.id, "Выбери группу 🔎")
                        .reply_markup(mit_group_keyboard()) // пример для МИТ
                        .await?;
                }
            }
            return Ok(());
        }

        // 2️⃣ Выбор факультета
        if let Ok(fac) = Faculty::from_str(&data) {
            info!("Пользователь {} выбрал факультет {:?}", q.from.id, fac);
            if fac == Faculty::Mit {
                bot.send_message(q.from.id, "Ты выбрал МИТ 📐. Теперь выбери форму обучения:")
                    .reply_markup(study_form_keyboard())
                    .await?;
            }
            return Ok(());
        }

        // 3️⃣ Выбор формы обучения
        if let Ok(form) = StudyForm::from_str(&data) {
            info!("Пользователь {} выбрал форму обучения {:?}", q.from.id, form);
            bot.send_message(q.from.id, format!("Форма обучения: {}. Теперь выбери курс:", form.title()))
                .reply_markup(course_keyboard())
                .await?;
            return Ok(());
        }

        // 4️⃣ Выбор курса
        if let Ok(course) = Course::from_str(&data) {
            info!("Пользователь {} выбрал курс {:?}", q.from.id, course);
            bot.send_message(q.from.id, format!("Курс: {}. Теперь выбери группу:", course.title()))
                .reply_markup(mit_group_keyboard())
                .await?;
            return Ok(());
        }

        // 5️⃣ Выбор группы (МИТ)
        if let Ok(group) = MitGroup::from_str(&data) {
            info!("Пользователь {} выбрал группу {:?}", q.from.id, group);

            // ⚠️ Здесь пока жёстко задаём факультет и форму, позже можно хранить состояние
            match db.register_student(q.from.id.0 as i64, "МИТ", group.title(), "Очная").await {
                Ok(student) => {
                    bot.send_message(q.from.id, format!(
                        "✅ Ты зарегистрирован!\nФакультет: {}\nФорма: {}\nГруппа: {}",
                        student.faculty, student.study_form, student.group_name
                    ))
                        .reply_markup(schedule_keyboard())
                        .await?;
                }
                Err(err) => {
                    error!("Ошибка регистрации: {:?}", err);
                    bot.send_message(q.from.id, "Ошибка при регистрации ❌").await?;
                }
            }
            return Ok(());
        }

        // 6️⃣ Неизвестный callback
        warn!("Получен неизвестный callback '{}' от пользователя {}", data, q.from.id);
    } else {
        warn!("handle_callback: пустой callback от пользователя {}", q.from.id);
    }
    Ok(())
}
