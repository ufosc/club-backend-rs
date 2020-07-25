table! {
    users (user_id) {
        user_id -> Int4,
        username -> Varchar,
        email -> Nullable<Varchar>,
        password_hash -> Nullable<Varchar>,
    }
}
