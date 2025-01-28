CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS orders (
	id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
	user_id UUID NOT NULL,
	product_id UUID NOT NULL,
	order_details_id UUID UNIQUE,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (product_id) REFERENCES products(id),
	FOREIGN KEY (order_details_id) REFERENCES order_details(id)
);


--	triggers

--	--	update timestamp	

	CREATE TRIGGER update_order_timestamp
	BEFORE UPDATE ON orders
	FOR EACH ROW
	EXECUTE FUNCTION update_updated_at();