CREATE TABLE posts (
    id              SERIAL PRIMARY KEY,
    user_id         INTEGER NOT NULL REFERENCES users(id),
    title           TEXT NOT NULL,
    content         TEXT NOT NULL
);

CREATE INDEX posts_user_id_key ON posts (user_id);
