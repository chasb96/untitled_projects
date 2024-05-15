CREATE TABLE project_snapshots (
    id SERIAL UNIQUE PRIMARY KEY,
    project_id VARCHAR(16) NOT NULL,
    version VARCHAR(16) NOT NULL,
    content JSON NOT NULL
);

CREATE INDEX idx_project_snapshots_project_id_version ON project_snapshots (project_id, version);