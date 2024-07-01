CREATE TABLE project_events (
    id SERIAL UNIQUE PRIMARY KEY,
    event_id VARCHAR(16) NOT NULL,
    project_id VARCHAR(16) NOT NULL,
    content JSON NOT NULL
);

CREATE INDEX project_events_event_id ON project_events (event_id);