// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    otps (id) {
        id -> Uuid,
        #[max_length = 255]
        phone -> Varchar,
        #[max_length = 12]
        code -> Varchar,
        payload -> Nullable<Jsonb>,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use postgis_diesel::sql_types::*;

    users (id) {
        id -> Uuid,
        #[max_length = 15]
        phone -> Varchar,
        #[max_length = 55]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        full_name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(otps, users,);
