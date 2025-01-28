CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
	id UUID NOT NULL PRIMARY KEY DEFAULT(uuid_generate_v4()),
	name VARCHAR(100) NOT NULL CHECK(name <> '') UNIQUE,
	email VARCHAR(255) NOT NULL CHECK(email <> '') UNIQUE,
	password VARCHAR(255) NOT NULL CHECK(password <> ''),
	sold INTEGER NOT NULL DEFAULT 0,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);



--	triggers

--	--	update timestamp	

CREATE TRIGGER update_users_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();