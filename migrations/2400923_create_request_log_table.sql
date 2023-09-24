CREATE TABLE log.requests(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    created_at TIMESTAMP DEFAULT NOW(),
    api_endpoint VARCHAR(200),
    status_code SMALLINT,
    status_message VARCHAR(200),
    content_length INT
);