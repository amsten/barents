CREATE SCHEMA IF NOT EXISTS ais;

-- CREATE TABLE ais.ais_latest_response_items (
--     id SERIAL PRIMARY KEY, -- Primary key column, auto-incrementing
--     type_field VARCHAR(255),
--     message_type BIGINT,
--     mmsi BIGINT,
--     msgtime TIMESTAMP WITH TIME ZONE,
--     imo_number BIGINT,
--     call_sign VARCHAR(255),
--     destination VARCHAR(255),
--     eta VARCHAR(255),
--     name VARCHAR(255),
--     draught INT,
--     ship_length INT,
--     ship_width INT,
--     ship_type INT,
--     dimension_a INT,
--     dimension_b INT,
--     dimension_c INT,
--     dimension_d INT,
--     position_fixing_device_type BIGINT,
--     report_class VARCHAR(255),
--     log_id UUID REFERENCES log.requests(id)
-- );
CREATE TABLE ais.ais_static_data (
                                     id SERIAL PRIMARY KEY,
                                     type_field VARCHAR(255),
                                     message_type BIGINT,
                                     mmsi BIGINT,
                                     msgtime TIMESTAMP WITH TIME ZONE,
                                     imo_number BIGINT,
                                     call_sign VARCHAR(255),
                                     destination VARCHAR(255),
                                     eta VARCHAR(255),
                                     name VARCHAR(255),
                                     draught INT,
                                     ship_length INT,
                                     ship_width INT,
                                     ship_type INT,
                                     dimension_a INT,
                                     dimension_b INT,
                                     dimension_c INT,
                                     dimension_d INT,
                                     position_fixing_device_type BIGINT,
                                     report_class VARCHAR(255),
                                     log_id UUID REFERENCES log.requests(id)
);

CREATE TABLE ais.ais_aton_data (
                                   id SERIAL PRIMARY KEY,
                                   type_field VARCHAR(255),
                                   message_type BIGINT,
                                   mmsi BIGINT,
                                   msgtime TIMESTAMP WITH TIME ZONE,
                                   dimension_a BIGINT,
                                   dimension_b BIGINT,
                                   dimension_c BIGINT,
                                   dimension_d BIGINT,
                                   type_of_aids_to_navigation BIGINT,
                                   latitude DOUBLE PRECISION,
                                   longitude DOUBLE PRECISION,
                                   name VARCHAR(255),
                                   type_of_electronic_fixing_device BIGINT,
                                   log_id UUID REFERENCES log.requests(id)
);

CREATE TABLE ais.ais_position_data (
                                       id SERIAL PRIMARY KEY,
                                       type_field VARCHAR(255),
                                       message_type BIGINT,
                                       course_over_ground DOUBLE PRECISION,
                                       ais_class VARCHAR(255),
                                       altitude DOUBLE PRECISION,
                                       latitude DOUBLE PRECISION,
                                       longitude DOUBLE PRECISION,
                                       navigational_status BIGINT,
                                       rate_of_turn BIGINT,
                                       speed_over_ground DOUBLE PRECISION,
                                       true_heading BIGINT,
                                       mmsi BIGINT,
                                       msgtime TIMESTAMP WITH TIME ZONE,
                                       log_id UUID REFERENCES log.requests(id)
);

