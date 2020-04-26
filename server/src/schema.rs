table! {
    use diesel::sql_types::*;
    use crate::common::types::*;

    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        status -> Post_status,
        published_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::common::types::*;

    users (id) {
        id -> Int4,
        email -> Varchar,
        name -> Varchar,
        status -> User_status,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
