use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use crate::domain::faculty::Faculty;
use crate::domain::buttons::TechButton;
use crate::domain::study_form::StudyForm;
use crate::domain::course::Course;
use crate::domain::groups::mit::MitGroup;

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

/// Клавиатура выбора факультета
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

/// Клавиатура выбора формы обучения
pub fn study_form_keyboard() -> InlineKeyboardMarkup {
    let forms = [StudyForm::FullTime, StudyForm::PartTime];
    InlineKeyboardMarkup::new(
        forms.iter()
            .map(|f| vec![InlineKeyboardButton::callback(f.title(), f.callback())])
            .collect::<Vec<_>>(),
    )
}

/// Клавиатура выбора курса
pub fn course_keyboard() -> InlineKeyboardMarkup {
    let courses = [Course::First, Course::Second, Course::Third, Course::Fourth];
    InlineKeyboardMarkup::new(
        courses.iter()
            .map(|c| vec![InlineKeyboardButton::callback(c.title(), c.callback())])
            .collect::<Vec<_>>(),
    )
}

/// Клавиатура выбора группы (пример для МИТ)
pub fn mit_group_keyboard() -> InlineKeyboardMarkup {
    let groups = [MitGroup::ISIT, MitGroup::PI, MitGroup::PInj, MitGroup::PM, MitGroup::UIR, MitGroup::MF];
    InlineKeyboardMarkup::new(
        groups.iter()
            .map(|g| vec![InlineKeyboardButton::callback(g.title(), g.callback())])
            .collect::<Vec<_>>(),
    )
}
