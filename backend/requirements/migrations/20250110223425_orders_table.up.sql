CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS orders (
	id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
	user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	product_id UUID NOT NULL REFERENCES products(id),
	products_number INTEGER NOT NULL CHECK(products_number >= 1),
	order_details_id UUID UNIQUE REFERENCES users(id) ON DELETE SET NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

--	triggers

--	--	update timestamp	

	CREATE TRIGGER update_order_timestamp
	BEFORE UPDATE ON orders
	FOR EACH ROW
	EXECUTE FUNCTION update_updated_at();