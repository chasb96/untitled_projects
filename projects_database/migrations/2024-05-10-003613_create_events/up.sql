CREATE TABLE project_events (
    id VARCHAR(64) UNIQUE PRIMARY KEY,
    content JSON NOT NULL
);