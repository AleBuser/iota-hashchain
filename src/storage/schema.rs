table! {
    block (id) {
        id -> Int4,
        timestamp -> Int4,
        hash -> Varchar,
        iota_link -> Varchar,
        previous_hash -> Varchar,
        previous_iota_link -> Varchar,
        data -> Text,
    }
}
