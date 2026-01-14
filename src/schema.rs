// @generated automatically by Diesel CLI.

diesel::table! {
    schedule_lessons (id) {
        id -> Int4,
        faculty -> Text,
        group_name -> Text,
        lesson_date -> Date,
        lesson_number -> Int4,
        subject -> Text,
        teacher -> Text,
        room -> Text,
    }
}

diesel::table! {
    students (id) {
        id -> Uuid,
        telegram_id -> Int8,
        faculty -> Text,
        group_name -> Text,
        created_at -> Timestamptz,
        study_form -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(schedule_lessons, students,);
