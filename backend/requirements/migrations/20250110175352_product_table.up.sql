CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS products (
	id UUID NOT NULL PRIMARY KEY DEFAULT(uuid_generate_v4()),
	name VARCHAR(255) NOT NULL UNIQUE,
	user_id UUID NOT NULL,
	description VARCHAR(255),
	price INTEGER NOT NULL,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);


--	triggers

--	--	update timestamp	

	CREATE TRIGGER update_products_timestamp
	BEFORE UPDATE ON products
	FOR EACH ROW
	EXECUTE FUNCTION update_updated_at();