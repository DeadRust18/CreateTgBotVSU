use sqlx::PgPool;
use crate::domain::student::Student;
use crate::db::students_repo;
use log::{info, debug, error};

/// DbFacade — фасад для работы с базой.
/// Все операции с БД идут через него.
#[derive(Clone)]
pub struct DbFacade {
    pool: PgPool,
}

impl DbFacade {
    /// Создаём новый фасад
    pub fn new(pool: PgPool) -> Self {
        info!("Создан новый DbFacade с пулом соединений.");
        Self { pool }
    }

    /// Найти студента по Telegram ID
    pub async fn find_student(&self, telegram_id: i64) -> anyhow::Result<Option<Student>> {
        debug!("Поиск студента с telegram_id={}", telegram_id);
        students_repo::find_by_telegram_id(&self.pool, telegram_id).await
    }

    /// Зарегистрировать нового студента
    ///
    /// - `telegram_id`: ID пользователя в Telegram
    /// - `faculty`: факультет
    /// - `group`: учебная группа
    /// - `study_form`: форма обучения ("очная" или "заочная")
    pub async fn register_student(
        &self,
        telegram_id: i64,
        faculty: &str,
        group: &str,
        study_form: &str,
    ) -> anyhow::Result<Student> {
        info!(
            "Регистрация студента: telegram_id={}, faculty={}, group={}, study_form={}",
            telegram_id, faculty, group, study_form
        );
        match students_repo::insert(&self.pool, telegram_id, faculty, group, study_form).await {
            Ok(student) => {
                info!(
                    "Студент с telegram_id={} успешно зарегистрирован (id={}).",
                    telegram_id, student.id
                );
                Ok(student)
            }
            Err(e) => {
                error!("Ошибка при регистрации студента telegram_id={}: {:?}", telegram_id, e);
                Err(e)
            }
        }
    }

    /// Обновить данные студента
    pub async fn update_student(
        &self,
        telegram_id: i64,
        faculty: &str,
        group: &str,
        study_form: &str,
    ) -> anyhow::Result<Option<Student>> {
        students_repo::update(&self.pool, telegram_id, faculty, group, study_form).await
    }

    /// Удалить студента
    pub async fn delete_student(&self, telegram_id: i64) -> anyhow::Result<u64> {
        students_repo::delete(&self.pool, telegram_id).await
    }
}
