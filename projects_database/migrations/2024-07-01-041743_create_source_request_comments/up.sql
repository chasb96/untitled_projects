CREATE TABLE source_request_comments (
    id SERIAL PRIMARY KEY,
    source_request_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);