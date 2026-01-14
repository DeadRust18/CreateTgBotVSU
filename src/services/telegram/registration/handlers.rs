use teloxide::prelude::*;
use crate::services::telegram::registration::keyboards::{
    faculty_keyboard, study_form_keyboard, course_keyboard, mit_group_keyboard,
};
use crate::domain::{faculty::Faculty, study_form::StudyForm, course::Course};
use crate::domain::groups::mit::MitGroup;
use crate::db::facade::DbFacade;
use std::sync::Arc;
use log::{info, error};

pub async fn handle_register(bot: Bot, q: CallbackQuery) -> Result<(), teloxide::RequestError> {
    bot.send_message(q.from.id, "Выбери факультет 📚")
        .reply_markup(faculty_keyboard())
        .await?;
    Ok(())
}

pub async fn handle_schedule(bot: Bot, q: CallbackQuery) -> Result<(), teloxide::RequestError> {
    bot.send_message(q.from.id, "Вот твоё расписание 📅").await?;
    Ok(())
}

pub async fn handle_choose_group(bot: Bot, q: CallbackQuery) -> Result<(), teloxide::RequestError> {
    bot.send_message(q.from.id, "Выбери группу 🔎")
        .reply_markup(mit_group_keyboard())
        .await?;
    Ok(())
}

pub async fn handle_faculty_choice(bot: Bot, q: CallbackQuery, faculty: Faculty) -> Result<(), teloxide::RequestError> {
    info!("Пользователь {} выбрал факультет {:?}", q.from.id, faculty);
    if faculty == Faculty::Mit {
        bot.send_message(q.from.id, "Ты выбрал МИТ 📐. Теперь выбери форму обучения:")
            .reply_markup(study_form_keyboard())
            .await?;
    }
    Ok(())
}

pub async fn handle_study_form(bot: Bot, q: CallbackQuery, form: StudyForm) -> Result<(), teloxide::RequestError> {
    bot.send_message(q.from.id, format!("Форма обучения: {}. Теперь выбери курс:", form.title()))
        .reply_markup(course_keyboard())
        .await?;
    Ok(())
}

pub async fn handle_course(bot: Bot, q: CallbackQuery, course: Course) -> Result<(), teloxide::RequestError> {
    bot.send_message(q.from.id, format!("Курс: {}. Теперь выбери группу:", course.title()))
        .reply_markup(mit_group_keyboard())
        .await?;
    Ok(())
}

pub async fn handle_group(bot: Bot, q: CallbackQuery, db: Arc<DbFacade>, group: MitGroup) -> Result<(), teloxide::RequestError> {
    match db.register_student(q.from.id.0 as i64, "МИТ", group.title(), "Очная").await {
        Ok(student) => {
            bot.send_message(q.from.id, format!(
                "✅ Ты зарегистрирован!\nФакультет: {}\nФорма: {}\nГруппа: {}",
                student.faculty, student.study_form, student.group_name
            ))
                .await?;
        }
        Err(err) => {
            error!("Ошибка регистрации: {:?}", err);
            bot.send_message(q.from.id, "Ошибка при регистрации ❌").await?;
        }
    }
    Ok(())
}
