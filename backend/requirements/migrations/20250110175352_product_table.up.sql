CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS products (
	id UUID NOT NULL PRIMARY KEY DEFAULT(uuid_generate_v4()),
	name VARCHAR(100) NOT NULL,
	user_id UUID NOT NULL,
	description VARCHAR(1000),
	price INTEGER NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);


--	triggers

--	--	update timestamp	

	CREATE TRIGGER update_products_timestamp
	BEFORE UPDATE ON products
	FOR EACH ROW
	EXECUTE FUNCTION update_updated_at();