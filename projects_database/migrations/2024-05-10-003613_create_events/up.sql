CREATE TABLE project_events (
    id SERIAL UNIQUE PRIMARY KEY,
    project_id VARCHAR(16) NOT NULL,
    content JSON NOT NULL
);