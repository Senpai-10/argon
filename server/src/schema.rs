// @generated automatically by Diesel CLI.

diesel::table! {
    albums (title) {
        title -> Text,
    }
}

diesel::table! {
    artists (name) {
        name -> Text,
    }
}

diesel::table! {
    songs (id) {
        id -> Text,
        title -> Text,
        artist_name -> Text,
        album_title -> Text,
        length -> Integer,
        year -> Nullable<Integer>,
        track -> Nullable<Integer>,
        path -> Text,
    }
}

diesel::joinable!(songs -> albums (album_title));
diesel::joinable!(songs -> artists (artist_name));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    artists,
    songs,
);
