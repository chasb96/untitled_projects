CREATE TABLE project_threads (
    id VARCHAR(16) PRIMARY KEY,
    project_id VARCHAR(16) NOT NULL,
    user_id VARCHAR(16) NOT NULL,
    title VARCHAR(128) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE project_thread_comments (
    id VARCHAR(16) PRIMARY KEY,
    thread_id VARCHAR(16) NOT NULL,
    user_id VARCHAR(16) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);