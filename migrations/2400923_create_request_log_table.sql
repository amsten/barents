CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE log.requests(
                             id UUID DEFAULT uuid_generate_v4() NOT NULL,
                             PRIMARY KEY (id),
                             created_at TIMESTAMP DEFAULT NOW(),
                             api_endpoint VARCHAR(200),
                             status_code INT,
                             status_message VARCHAR(200),
                             number_of_messages_received BIGINT
);
