CREATE TABLE project_metrics (
    id SERIAL PRIMARY KEY,
    project_id VARCHAR(16) NOT NULL UNIQUE,
    view_count INTEGER NOT NULL DEFAULT 0
)