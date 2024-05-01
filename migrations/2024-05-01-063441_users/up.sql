CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE user_privilege AS ENUM ('free', 'super', 'vip');

CREATE TABLE users (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    privilege user_privilege NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS users_id_uindex
    on users (id);
