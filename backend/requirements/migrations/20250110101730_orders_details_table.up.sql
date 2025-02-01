CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS order_details (
	id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
	delivery_address VARCHAR(255) NOT NULL,
	created_at TIMESTAMPTZ DEFAULT NOW(),
	updated_at TIMESTAMPTZ DEFAULT NOW()
);


--	triggers

--	--	update timestamp	

	CREATE TRIGGER update_order_details_timestamp
	BEFORE UPDATE ON order_details
	FOR EACH ROW
	EXECUTE FUNCTION update_updated_at();