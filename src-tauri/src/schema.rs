// @generated automatically by Diesel CLI.

diesel::table! {
    categories (name) {
        name -> Text,
    }
}

diesel::table! {
    subtasks (id) {
        id -> Text,
        parent_id -> Text,
    }
}

diesel::table! {
    tasks (id) {
        id -> Text,
        category_name -> Text,
        title -> Text,
        desc -> Nullable<Text>,
        status -> Integer,
        priority -> Integer,
        is_child_task -> Integer,
        done_at -> Nullable<Integer>,
        created_at -> Integer,
        updated_at -> Integer,
    }
}

diesel::joinable!(tasks -> categories (category_name));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    subtasks,
    tasks,
);
