// @generated automatically by Diesel CLI.

diesel::table! {
    patients (id) {
        id -> Uuid,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        created_at -> Nullable<Timestamptz>,
    }
}
