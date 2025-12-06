// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Integer,
        message_type -> Text,
        transmission_type -> Nullable<Text>,
        session_id -> Text,
        aircraft_id -> Text,
        hex_ident -> Text,
        flight_id -> Text,
        generated_timestamp -> BigInt,
        logged_timestamp -> BigInt,
        callsign -> Nullable<Text>,
        altitude -> Nullable<BigInt>,
        ground_speed -> Nullable<BigInt>,
        track -> Nullable<BigInt>,
        latitude -> Nullable<Double>,
        longitude -> Nullable<Double>,
        vertical_rate -> Nullable<BigInt>,
        squawk -> Nullable<Text>,
        alert -> Nullable<Bool>,
        emergency -> Nullable<Bool>,
        spi -> Nullable<Bool>,
        is_on_ground -> Nullable<Bool>,
    }
}
