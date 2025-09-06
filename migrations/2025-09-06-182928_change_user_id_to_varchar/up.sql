ALTER TABLE to_do DROP COLUMN user_id;
ALTER TABLE to_do ADD COLUMN user_id VARCHAR NOT NULL DEFAULT 'default_id';
ALTER TABLE to_do ADD CONSTRAINT to_do_user_id_fkey FOREIGN KEY (user_id) REFERENCES users (unique_id);
