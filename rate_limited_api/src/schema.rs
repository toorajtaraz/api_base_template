table! {
    ips (id) {
        id -> Int4,
        ip -> Cidr,
        url_path -> Text,
        last_access -> Timestamp,
        first_access -> Timestamp,
        access_count -> Int4,
    }
}

table! {
    tokens (id) {
        id -> Int4,
        user_id -> Int4,
    }
}

table! {
    urls (url_path) {
        url_path -> Text,
        limit_per -> Int4,
        limit_count -> Int4,
        access_level -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
        access_level -> Int4,
        is_deleted -> Bool,
        created_at -> Timestamp,
    }
}

joinable!(ips -> urls (url_path));
joinable!(tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(ips, tokens, urls, users,);
