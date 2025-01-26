-- Add up migration script here
CREATE TABLE IF NOT EXISTS "orders" (
	"order_id" SERIAL PRIMARY KEY,
	"user_id" INTEGER NOT NULL,
	"product_id" INTEGER NOT NULL,
	"order_details_id" INTEGER UNIQUE,
	"created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY ("user_id") REFERENCES users(user_id) ON DELETE CASCADE,
	FOREIGN KEY ("product_id") REFERENCES products(product_id),
	FOREIGN KEY ("order_details_id") REFERENCES order_details(order_details_id)
);


--	triggers

--	--	update timestamp	

	CREATE TRIGGER update_order_timestamp
	BEFORE UPDATE ON "orders"
	FOR EACH ROW
	EXECUTE FUNCTION update_updated_at();