// src/db/facade.rs

// PgPool — это "пул соединений" к PostgreSQL.
// Он хранит несколько открытых подключений к базе, чтобы не создавать новое каждый раз.
use sqlx::PgPool;

// Подключаем структуру Student — это наша модель студента в коде.
use crate::domain::student::Student;

// Подключаем модуль students_repo — там лежат функции для работы с таблицей users (поиск, вставка).
use crate::db::students_repo;
use log::{info, debug, error};

/// DbFacade — это "фасад" для работы с базой.
/// Представь, что это как "единая дверь": вместо того чтобы напрямую дергать SQL-запросы,
/// мы обращаемся к удобным методам фасада.
#[derive(Clone)]
pub struct DbFacade {
    pool: PgPool, // внутри хранится пул соединений к базе
}

impl DbFacade {
    /// Создаём новый фасад.
    /// Принимаем пул соединений и сохраняем его внутри структуры.
    /// Теперь через этот фасад можно вызывать методы для работы с базой.
    pub fn new(pool: PgPool) -> Self {
        info!("Создан новый DbFacade с пулом соединений.");
        Self { pool }
    }

    /// Метод `find_student` ищет студента по его Telegram ID.
    ///
    /// - Аргумент `telegram_id`: уникальный номер студента в Telegram.
    /// - Возвращает либо `Some(Student)`, если студент найден,
    ///   либо `None`, если такого студента нет.
    ///
    /// Внутри метод просто вызывает функцию `find_by_telegram_id` из `students_repo`,
    /// передавая ей пул соединений и нужный ID.
    pub async fn find_student(&self, telegram_id: i64) -> anyhow::Result<Option<Student>> {
        debug!("Поиск студента с telegram_id={}", telegram_id);
        match students_repo::find_by_telegram_id(&self.pool, telegram_id).await {
            Ok(result) => {
                if result.is_some() {
                    info!("Студент с telegram_id={} найден.", telegram_id);
                } else {
                    info!("Студент с telegram_id={} не найден.", telegram_id);
                }
                Ok(result)
            }
            Err(e) => {
                error!("Ошибка при поиске студента с telegram_id={}: {:?}", telegram_id, e);
                Err(e)
            }
        }
    }

    /// Метод `register_student` регистрирует нового студента в базе.
    ///
    /// - Аргументы:
    ///   - `telegram_id`: номер студента в Telegram.
    ///   - `faculty`: факультет, где он учится.
    ///   - `group`: название учебной группы.
    ///
    /// - Возвращает структуру `Student` с заполненными данными:
    ///   включая автоматически созданный `id` и время `created_at`.
    ///
    /// Внутри метод вызывает функцию `insert` из `students_repo`,
    /// которая делает SQL-запрос INSERT и возвращает нового студента.
    pub async fn register_student(
        &self,
        telegram_id: i64,
        faculty: &str,
        group: &str,
    ) -> anyhow::Result<Student> {
        info!(
            "Регистрация студента: telegram_id={}, faculty={}, group={}",
            telegram_id, faculty, group
        );
        match students_repo::insert(&self.pool, telegram_id, faculty, group).await {
            Ok(student) => {
                info!("Студент с telegram_id={} успешно зарегистрирован (id={}).", telegram_id, student.id);
                Ok(student)
            }
            Err(e) => {
                error!("Ошибка при регистрации студента telegram_id={}: {:?}", telegram_id, e);
                Err(e)
            }
        }
    }
}