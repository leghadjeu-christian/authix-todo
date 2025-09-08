-- Drop new tables
DROP TABLE IF EXISTS to_do CASCADE;
DROP TABLE IF EXISTS users CASCADE;

-- Recreate old users with SERIAL id
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    unique_id VARCHAR NOT NULL,
    UNIQUE (email),
    UNIQUE (username)
);

-- Recreate old to_do with integer FK
CREATE TABLE to_do (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users(id)
);

ALTER TABLE to_do
ADD CONSTRAINT uc_item UNIQUE (title, user_id);

-- Insert placeholder again
INSERT INTO users (username, email, password, unique_id)
VALUES ('placeholder', 'placeholder email', 'placeholder password', 'placeholder unique id');
