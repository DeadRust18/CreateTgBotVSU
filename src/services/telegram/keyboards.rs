use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

/// Клавиатура для незарегистрированного пользователя
pub fn registration_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback("Зарегистрироваться", "register"),
    ]])
}

/// Клавиатура для зарегистрированного пользователя
pub fn schedule_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("Моё расписание", "my_schedule")],
        vec![InlineKeyboardButton::callback("Выбрать группу", "choose_group")],
    ])
}

/// Клавиатура выбора факультета (пример)
pub fn faculty_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("Факультет математики", "faculty_math")],
        vec![InlineKeyboardButton::callback("Факультет иностранных граждан", "faculty_foreign")],
    ])
}
