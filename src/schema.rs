// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Text,
        title -> Text,
        artist_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    artists (id) {
        id -> Text,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    features (id) {
        id -> Text,
        artist_id -> Text,
        track_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    scan_info (id) {
        id -> Int4,
        scan_start -> Timestamp,
        scan_end -> Timestamp,
        artists -> Int4,
        albums -> Int4,
        tracks -> Int4,
    }
}

diesel::table! {
    sessions (id) {
        id -> Text,
        user_id -> Text,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    tracks (id) {
        id -> Text,
        title -> Text,
        artist_id -> Nullable<Text>,
        album_id -> Nullable<Text>,
        duration -> Int4,
        year -> Nullable<Int4>,
        track_number -> Nullable<Int4>,
        last_play -> Nullable<Timestamp>,
        plays -> Int4,
        path -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        name -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(albums -> artists (artist_id));
diesel::joinable!(features -> artists (artist_id));
diesel::joinable!(features -> tracks (track_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(tracks -> albums (album_id));
diesel::joinable!(tracks -> artists (artist_id));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    artists,
    features,
    scan_info,
    sessions,
    tracks,
    users,
);
