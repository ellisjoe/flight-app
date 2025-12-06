CREATE TABLE messages
(
    id                  INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    message_type        TEXT    NOT NULL,
    transmission_type   TEXT    NOT NULL,
    session_id          TEXT    NOT NULL,
    aircraft_id         TEXT    NOT NULL,
    hex_ident           TEXT    NOT NULL,
    flight_id           TEXT    NOT NULL,
    generated_timestamp BIGINT NOT NULL,
    logged_timestamp    BIGINT NOT NULL,

    callsign            TEXT,
    altitude            BIGINT,
    ground_speed        BIGINT,
    track               BIGINT,
    latitude            DOUBLE,
    longitude           DOUBLE,
    vertical_rate       BIGINT,
    squawk              TEXT,
    alert               BOOL,
    emergency           BOOL,
    spi                 BOOL,
    is_on_ground        BOOL
);

CREATE INDEX messages_timestamp
ON messages (generated_timestamp);

CREATE INDEX messages_type_timestamp
ON messages (message_type, generated_timestamp);