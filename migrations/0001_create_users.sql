CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS refresh_tokens (
    id SERIAL PRIMARY KEY,
    token REAL NOT NULL,
    user SERIAL NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY (user) 
        REFERENCES users(id)
        ON DELETE CASCADE
)