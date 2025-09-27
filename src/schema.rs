// @generated automatically by Diesel CLI.

diesel::table! {
    category (id) {
        id -> Int2,
        name -> Text,
        alias -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    news (id) {
        id -> Uuid,
        title -> Text,
        description -> Nullable<Text>,
        author_id -> Uuid,
        category_id -> Int2,
        thumbnail -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    roles (id) {
        id -> Int2,
        name -> Text,
        alias -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Text,
        password -> Text,
        full_name -> Nullable<Text>,
        role_id -> Int2,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(news -> category (category_id));
diesel::joinable!(news -> users (author_id));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(category, news, roles, users,);
