CREATE TABLE project_tags (
    id SERIAL PRIMARY KEY,
    project_id VARCHAR(16) NOT NULL,
    tag VARCHAR(16) NOT NULL
)