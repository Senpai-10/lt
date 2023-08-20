// @generated automatically by Diesel CLI.

diesel::table! {
    categories (name) {
        name -> Text,
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
        creation_date -> Integer,
        completion_date -> Nullable<Integer>,
        modification_date -> Integer,
    }
}

diesel::joinable!(tasks -> categories (category_name));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    tasks,
);
