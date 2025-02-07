CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS products (
	id UUID NOT NULL PRIMARY KEY DEFAULT(uuid_generate_v4()),
	name VARCHAR(100) NOT NULL CHECK(name <> ''),
	user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	description VARCHAR(1000),
	price_in_cents BIGINT NOT NULL CHECK(price_in_cents >= 0),
	number_in_stock INTEGER NOT NULL CHECK(number_in_stock < 1000),
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);



	CREATE OR REPLACE FUNCTION initial_number_in_stock() 
	RETURNS TRIGGER AS $$
	BEGIN
		-- Vérifie si l'utilisateur essaie de commander son propre produit
		IF NEW .number_in_stock < 1
			THEN RAISE EXCEPTION 'positive_initial_number_in_stock';
		END IF;
		RETURN NEW;
	END;
	$$ LANGUAGE plpgsql;
	--  --
	CREATE TRIGGER check_initial_number_in_stock
	BEFORE INSERT OR UPDATE ON products
	FOR EACH ROW
	EXECUTE FUNCTION initial_number_in_stock();

--	--	update timestamp	

	CREATE TRIGGER update_products_timestamp
	BEFORE UPDATE ON products
	FOR EACH ROW
	EXECUTE FUNCTION update_updated_at();