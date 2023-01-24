// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Int4,
        api_key -> Varchar,
        salt -> Varchar,
        active -> Bit,
    }
}

diesel::table! {
    algorithm (id) {
        id -> Int4,
        crypto -> Varchar,
        salting -> Varchar,
    }
}

diesel::table! {
    categoria (id) {
        id -> Varchar,
        is_unsafe -> Bit,
        salt -> Varchar,
        owner -> Int4,
    }
}

diesel::table! {
    hash (id) {
        id -> Varchar,
        is_unsafe -> Bit,
        salt -> Varchar,
        hashed_data -> Varchar,
        owner -> Varchar,
    }
}

diesel::table! {
    user_algorithm (algorithm_id, user_id) {
        user_id -> Int4,
        algorithm_id -> Int4,
        ordering -> Int4,
    }
}

diesel::joinable!(categoria -> account (owner));
diesel::joinable!(hash -> categoria (owner));
diesel::joinable!(user_algorithm -> account (user_id));
diesel::joinable!(user_algorithm -> algorithm (algorithm_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    algorithm,
    categoria,
    hash,
    user_algorithm,
);
