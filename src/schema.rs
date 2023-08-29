// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Text,
        title -> Text,
        artist_name -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    artists (name) {
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    features (id) {
        id -> Text,
        artist_name -> Text,
        track_id -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
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
    tracks (id) {
        id -> Text,
        title -> Text,
        artist_name -> Nullable<Text>,
        album_id -> Nullable<Text>,
        duration -> Int4,
        year -> Nullable<Int4>,
        track_number -> Nullable<Int4>,
        last_play -> Nullable<Timestamp>,
        plays -> Int4,
        path -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(albums -> artists (artist_name));
diesel::joinable!(features -> artists (artist_name));
diesel::joinable!(features -> tracks (track_id));
diesel::joinable!(tracks -> albums (album_id));
diesel::joinable!(tracks -> artists (artist_name));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    artists,
    features,
    scan_info,
    tracks,
);
