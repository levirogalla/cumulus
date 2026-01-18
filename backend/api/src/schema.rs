// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Int4,
        name -> Varchar,
        date_created -> Nullable<Date>,
    }
}

diesel::table! {
    albums_media (id) {
        id -> Int4,
        media -> Varchar,
        album -> Int4,
        date_added -> Nullable<Date>,
    }
}

diesel::table! {
    media (key) {
        key -> Varchar,
        notes -> Nullable<Varchar>,
        date_uploaded -> Nullable<Date>,
    }
}

diesel::joinable!(albums_media -> albums (album));
diesel::joinable!(albums_media -> media (media));

diesel::allow_tables_to_appear_in_same_query!(albums, albums_media, media,);
