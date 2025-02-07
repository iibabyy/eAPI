CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS orders (
	id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
	user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	product_id UUID NOT NULL REFERENCES products(id),
	products_number INTEGER NOT NULL CHECK(
		products_number >= 1 AND
		products_number <= 1000
	),
	order_details_id UUID UNIQUE REFERENCES users(id) ON DELETE SET NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

--	function/triggers

	--	--	check user != product.user

	CREATE OR REPLACE FUNCTION check_if_product_belong_to_buyer() 
	RETURNS TRIGGER AS $$
	BEGIN
		-- Vérifie si l'utilisateur essaie de commander son propre produit
		IF EXISTS (
			SELECT 1
			FROM products
			WHERE id = NEW.product_id AND user_id = NEW.user_id
		) THEN
			RAISE EXCEPTION 'auto-buying';
		END IF;
		RETURN NEW;
	END;
	$$ LANGUAGE plpgsql;
	--  --
	CREATE TRIGGER check_user_product
	BEFORE INSERT OR UPDATE ON orders
	FOR EACH ROW
	EXECUTE FUNCTION check_if_product_belong_to_buyer();


	--	--	update timestamp	

	CREATE TRIGGER update_order_timestamp
	BEFORE UPDATE ON orders
	FOR EACH ROW
	EXECUTE FUNCTION update_updated_at();
