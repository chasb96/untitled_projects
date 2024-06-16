CREATE TABLE project_threads (
    id SERIAL PRIMARY KEY,
    project_id VARCHAR(16) NOT NULL,
    user_id INTEGER NOT NULL,
    title VARCHAR(128) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE project_thread_comments (
    id SERIAL PRIMARY KEY,
    thread_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);