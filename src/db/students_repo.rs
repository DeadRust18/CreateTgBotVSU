// src/db/students_repo.rs

// Подключаем библиотеку для работы с базой данных PostgreSQL через пул соединений.
// "Пул" — это как очередь готовых подключений к базе, чтобы не открывать новое каждый раз.
use sqlx::PgPool;

// Подключаем нашу структуру Student, которая описывает студента в коде.
use crate::domain::student::Student;


/// Функция ищет студента по его Telegram ID.
/// Представь: у каждого студента есть свой "номер" в Telegram, и мы хотим найти его запись в базе.
///
/// - `pool`: это "пул соединений" с базой данных.
/// - `telegram_id`: номер студента в Telegram.
/// Возвращает либо найденного студента, либо `None` (если такого нет).
pub async fn find_by_telegram_id(pool: &PgPool, telegram_id: i64) -> anyhow::Result<Option<Student>> {
    // Делаем SQL-запрос: ищем строку в таблице "users", где колонка telegram_id равна нашему значению.
    let row = sqlx::query!(
        r#"
        SELECT id, telegram_id, faculty, group_name, created_at
        FROM users
        WHERE telegram_id = $1
        "#,
        telegram_id // ← сюда подставляется значение из аргумента функции
    )
        // fetch_optional значит: либо вернётся строка, либо ничего (None).
        .fetch_optional(pool)
        .await?;

    // Если строка найдена, превращаем её в структуру Student.
    // Если нет — вернётся None.
    Ok(row.map(|r| Student {
        id: r.id,                 // уникальный идентификатор (UUID)
        telegram_id: r.telegram_id, // номер в Telegram
        faculty: r.faculty,       // факультет
        group_name: r.group_name, // название группы
        created_at: r.created_at, // дата и время создания записи
    }))
}

/// Функция добавляет нового студента в базу.
/// Представь: студент только что пришёл, и мы записываем его данные.
///
/// - `pool`: пул соединений с базой.
/// - `telegram_id`: номер студента в Telegram.
/// - `faculty`: факультет.
/// - `group_name`: название группы.
/// Возвращает созданного студента (с уже присвоенным ID и временем создания).
pub async fn insert(pool: &PgPool, telegram_id: i64, faculty: &str, group_name: &str) -> anyhow::Result<Student> {
    // Делаем SQL-запрос: вставляем новую строку в таблицу "users".
    // RETURNING значит: сразу вернуть все нужные поля новой записи.
    let row = sqlx::query!(
        r#"
        INSERT INTO users (telegram_id, faculty, group_name)
        VALUES ($1, $2, $3)
        RETURNING id, telegram_id, faculty, group_name, created_at
        "#,
        telegram_id, // ← номер в Telegram
        faculty,     // ← факультет
        group_name   // ← группа
    )
        // fetch_one значит: мы точно ожидаем одну строку (нового студента).
        .fetch_one(pool)
        .await?;

    // Превращаем результат запроса в структуру Student.
    Ok(Student {
        id: row.id,                 // уникальный идентификатор (UUID)
        telegram_id: row.telegram_id, // номер в Telegram
        faculty: row.faculty,       // факультет
        group_name: row.group_name, // группа
        created_at: row.created_at, // дата и время создания записи
    })
}
