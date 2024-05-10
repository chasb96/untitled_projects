CREATE TABLE project_events (
    id SERIAL NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    content JSON NOT NULL
);