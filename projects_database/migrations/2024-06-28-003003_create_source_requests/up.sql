CREATE TABLE project_source_requests (
    id SERIAL UNIQUE PRIMARY KEY,
    project_id VARCHAR(16) NOT NULL,
    user_id INT NOT NULL,
    state SMALLINT NOT NULL,
    content JSON NOT NULL
);