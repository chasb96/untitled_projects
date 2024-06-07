CREATE EXTENSION pg_trgm;
CREATE EXTENSION fuzzystrmatch;

CREATE TABLE projects_search (
    code VARCHAR(32) NOT NULL,
    value VARCHAR(32) NOT NULL,
    project_id VARCHAR(16) NOT NULL,
    name VARCHAR(32) NOT NULL
);

CREATE INDEX idx_projects_search_value_trgm ON projects_search USING gin (value gin_trgm_ops);