use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use crate::domain::faculty::Faculty;
use crate::domain::buttons::TechButton;

/// Клавиатура для незарегистрированного пользователя
pub fn registration_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback(
            TechButton::Register.title(),
            TechButton::Register.callback(),
        ),
    ]])
}

/// Клавиатура для зарегистрированного пользователя
pub fn schedule_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            TechButton::MySchedule.title(),
            TechButton::MySchedule.callback(),
        )],
        vec![InlineKeyboardButton::callback(
            TechButton::ChooseGroup.title(),
            TechButton::ChooseGroup.callback(),
        )],
    ])
}

/// Клавиатура выбора факультета (пример)
pub fn faculty_keyboard() -> InlineKeyboardMarkup {
    let faculties = [
        Faculty::Mit,
        Faculty::Hbg,
        Faculty::Ped,
        Faculty::Spp,
        Faculty::Foreign,
        Faculty::Fks,
        Faculty::Hzk,
        Faculty::Hgf,
        Faculty::Law,
    ];

    InlineKeyboardMarkup::new(
        faculties
            .iter()
            .map(|f| vec![InlineKeyboardButton::callback(f.title(), f.callback())])
            .collect::<Vec<_>>(),
    )
}