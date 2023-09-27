CREATE SCHEMA IF NOT EXISTS ais;

CREATE TABLE ais.ais_latest_response_items (
    id SERIAL PRIMARY KEY, -- Primary key column, auto-incrementing
    type_field VARCHAR(255),
    message_type BIGINT,
    mmsi BIGINT,
    msgtime TIMESTAMP WITH TIME ZONE,
    imo_number BIGINT,
    call_sign VARCHAR(255),
    destination VARCHAR(255),
    eta VARCHAR(255),
    name VARCHAR(255),
    draught BIGINT,
    ship_length BIGINT,
    ship_width BIGINT,
    ship_type BIGINT,
    dimension_a BIGINT,
    dimension_b BIGINT,
    dimension_c BIGINT,
    dimension_d BIGINT,
    position_fixing_device_type BIGINT,
    report_class VARCHAR(255),
    log_id UUID REFERENCES log.requests(id)
);
