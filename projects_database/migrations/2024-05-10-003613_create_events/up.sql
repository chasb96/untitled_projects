CREATE TABLE project_events (
    id SERIAL UNIQUE PRIMARY KEY,
    event_id VARCHAR(64) NOT NULL,
    project_id VARCHAR(16) NOT NULL,
    content JSON NOT NULL
);