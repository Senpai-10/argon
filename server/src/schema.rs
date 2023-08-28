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
    features (id) {
        id -> Text,
        artist_name -> Text,
        song_id -> Text,
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

diesel::joinable!(features -> artists (artist_name));
diesel::joinable!(features -> songs (song_id));
diesel::joinable!(songs -> albums (album_title));
diesel::joinable!(songs -> artists (artist_name));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    artists,
    features,
    songs,
);
