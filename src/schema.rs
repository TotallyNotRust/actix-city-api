// @generated automatically by Diesel CLI.

diesel::table! {
    City (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        country_id -> Nullable<Smallint>,
    }
}

diesel::table! {
    CityLanguage (city_id, language_id) {
        city_id -> Integer,
        language_id -> Integer,
    }
}

diesel::table! {
    Country (id) {
        id -> Smallint,
        name -> Text,
    }
}

diesel::table! {
    Language (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    PointOfInterest (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        city_id -> Integer,
    }
}

diesel::joinable!(City -> Country (country_id));
diesel::joinable!(CityLanguage -> City (city_id));
diesel::joinable!(CityLanguage -> Language (language_id));
diesel::joinable!(PointOfInterest -> City (city_id));

diesel::allow_tables_to_appear_in_same_query!(
    City,
    CityLanguage,
    Country,
    Language,
    PointOfInterest,
);
