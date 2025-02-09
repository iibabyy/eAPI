CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
	id UUID NOT NULL PRIMARY KEY DEFAULT(uuid_generate_v4()),
	name VARCHAR(100) NOT NULL CHECK(name <> ''),
	email VARCHAR(255) NOT NULL CHECK(email <> '') UNIQUE,
	password VARCHAR(255) NOT NULL CHECK(password <> ''),
	last_token_id VARCHAR(255) DEFAULT NULL,
	sold_in_cents BIGINT NOT NULL DEFAULT 0 CHECK(sold_in_cents >= 0),
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);



--	triggers

--	--	update timestamp	

CREATE TRIGGER update_users_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();