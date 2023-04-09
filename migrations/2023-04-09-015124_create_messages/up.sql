-- Your SQL goes here
CREATE TABLE messages (
    id INT PRIMARY KEY NOT NULL,
    content TEXT NOT NULL,
    role VARCHAR(255) NOT NULL,
    finish_reason VARCHAR(255) NOT NULL,
    prompt_tokens INT NOT NULL,
    completion_tokens INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
