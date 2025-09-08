-- 1. Drop the to_do table if it exists (this removes FKs too)
DROP TABLE IF EXISTS to_do CASCADE;

-- 2. Drop the users table if it exists
DROP TABLE IF EXISTS users CASCADE;

-- 3. Recreate users with TEXT PK (Keycloak sub)
CREATE TABLE users (
    id TEXT PRIMARY KEY,            -- Keycloak sub (string ID)
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,      -- optional with Keycloak
    UNIQUE (email),
    UNIQUE (username)
);

-- Insert a placeholder user (string ID now)
INSERT INTO users (id, username, email, password)
VALUES ('placeholder-id', 'placeholder', 'placeholder email', 'placeholder password');

-- 4. Recreate to_do with FK to users(id)
CREATE TABLE to_do (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id)  -- FK points to TEXT user id
);

-- 5. Add unique constraint for per-user items
ALTER TABLE to_do
ADD CONSTRAINT uc_item UNIQUE (title, user_id);
