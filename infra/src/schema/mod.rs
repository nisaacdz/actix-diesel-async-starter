// @generated automatically by Diesel CLI.

diesel::table! {
    otps (id) {
        id -> Uuid,
        user_id -> Uuid,
        code -> Varchar,
        expires_at -> Timestamptz,
        used_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password_hash -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(otps -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(otps, users,);
