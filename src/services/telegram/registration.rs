use teloxide::prelude::*;
use crate::db::facade::DbFacade;
use crate::domain::faculty::Faculty;
use crate::services::telegram::keyboards::study_form_keyboard;
use std::sync::Arc;
use log::{info, error};

/// Обработка выбора факультета
pub async fn handle_faculty_choice(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
    faculty: Faculty,
) -> Result<(), teloxide::RequestError> {
    info!("Пользователь {} выбрал факультет {:?}", q.from.id, faculty);

    // После выбора факультета предлагаем выбрать форму обучения
    bot.send_message(q.from.id, format!("Ты выбрал: {}. Теперь выбери форму обучения:", faculty.title()))
        .reply_markup(study_form_keyboard())
        .await?;

    Ok(())
}

/// Обработка выбора формы обучения
pub async fn handle_study_form(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
    faculty: &str,
    study_form: &str,
) -> Result<(), teloxide::RequestError> {
    match db.register_student(q.from.id.0 as i64, faculty, "ИСИТ", study_form).await {
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
