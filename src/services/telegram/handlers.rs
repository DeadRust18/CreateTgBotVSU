use teloxide::prelude::*;
use crate::db::facade::DbFacade;
use crate::domain::buttons::TechButton;
use crate::domain::faculty::Faculty;
use crate::domain::study_form::StudyForm;
use crate::domain::course::Course;
use crate::domain::groups::mit::MitGroup;
use crate::services::telegram::keyboards::{registration_keyboard, schedule_keyboard};
use crate::services::telegram::registration::handlers as reg_handlers;
use std::sync::Arc;
use log::{info, debug, warn, error};
use std::str::FromStr;

pub async fn handle_message(
    bot: Bot,
    msg: Message,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    let telegram_id = msg.from.map(|u| u.id.0).unwrap_or(0);
    debug!("handle_message: получен запрос от пользователя {}", telegram_id);

    match db.find_student(telegram_id.try_into().unwrap()).await {
        Ok(Some(student)) => {
            info!("Пользователь {} найден: {}", telegram_id, student.group_name);
            bot.send_message(msg.chat.id, format!("Привет, {}!", student.group_name))
                .reply_markup(schedule_keyboard())
                .await?;
        }
        Ok(None) => {
            warn!("Пользователь {} не найден, предлагаем регистрацию", telegram_id);
            bot.send_message(msg.chat.id, "Ты ещё не зарегистрирован. Хочешь пройти регистрацию?")
                .reply_markup(registration_keyboard())
                .await?;
        }
        Err(err) => {
            error!("Ошибка поиска студента {}: {:?}", telegram_id, err);
            bot.send_message(msg.chat.id, "Ошибка при обращении к базе ❌").await?;
        }
    }
    Ok(())
}

pub async fn handle_callback(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    if let Some(ref data) = q.data {
        debug!("handle_callback: callback='{}' от {}", data, q.from.id);

        if let Ok(btn) = TechButton::from_str(&data) {
            match btn {
                TechButton::Register => reg_handlers::handle_register(bot.clone(), q.clone()).await?,
                TechButton::MySchedule => reg_handlers::handle_schedule(bot.clone(), q.clone()).await?,
                TechButton::ChooseGroup => reg_handlers::handle_choose_group(bot.clone(), q.clone()).await?,
            }
            return Ok(());
        }

        if let Ok(fac) = Faculty::from_str(&data) {
            reg_handlers::handle_faculty_choice(bot, q, fac).await?;
            return Ok(());
        }

        if let Ok(form) = StudyForm::from_str(&data) {
            reg_handlers::handle_study_form(bot, q, form).await?;
            return Ok(());
        }

        if let Ok(course) = Course::from_str(&data) {
            reg_handlers::handle_course(bot, q, course).await?;
            return Ok(());
        }

        if let Ok(group) = MitGroup::from_str(&data) {
            reg_handlers::handle_group(bot, q, db.clone(), group).await?;
            return Ok(());
        }

        warn!("Неизвестный callback '{}' от {}", data, q.from.id);
    }
    Ok(())
}
