-- Add down migration script here

DROP TYPE IF EXISTS user_role;

DROP EXTENSION IF EXISTS "uuid-ossp";

DROP TABLE IF EXISTS "users";
