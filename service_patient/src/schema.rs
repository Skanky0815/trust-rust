// @generated automatically by Diesel CLI.

diesel::table! {
    patients (id) {
        id -> Uuid,
        #[max_length = 255]
        external_id -> Nullable<Varchar>,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        date_of_birth -> Nullable<Date>,
        #[max_length = 10]
        insurance_number -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
