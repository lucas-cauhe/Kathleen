table! {
    contributors (id) {
        id -> Int4,
        contributor -> Varchar,
        contributor_url -> Text,
    }
}

table! {
    languages (id) {
        id -> Int4,
        language_name -> Varchar,
    }
}

table! {
    repos (id) {
        id -> Int4,
        repo_name -> Text,
        repo_url -> Text,
        languages_id -> Array<Int4>,
        contributors_id -> Array<Int4>,
    }
}

allow_tables_to_appear_in_same_query!(
    contributors,
    languages,
    repos,
);
