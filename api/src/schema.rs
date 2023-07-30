// @generated automatically by Diesel CLI.

diesel::table! {
    comment (id) {
        id -> Int4,
        data -> Text,
        story_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    story (id) {
        id -> Int4,
        data -> Text,
    }
}

diesel::table! {
    user_account (id) {
        id -> Int4,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 100]
        password -> Varchar,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
    }
}

diesel::table! {
    user_story (id) {
        id -> Int4,
        story_id -> Int4,
        user_id -> Int4,
    }
}

diesel::joinable!(comment -> story (story_id));
diesel::joinable!(comment -> user_account (user_id));
diesel::joinable!(user_story -> story (story_id));
diesel::joinable!(user_story -> user_account (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comment,
    story,
    user_account,
    user_story,
);
