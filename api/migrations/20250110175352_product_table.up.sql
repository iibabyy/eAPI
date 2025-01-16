-- Add up migration script here
CREATE TABLE IF NOT EXISTS "products" (
	"product_id" SERIAL UNIQUE,
	"name" VARCHAR(255) NOT NULL,
	"user_id" INTEGER NOT NULL,
	"description" VARCHAR(255),
	"price" INTEGER NOT NULL,
	"created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY("user_id") REFERENCES users(user_id) ON DELETE CASCADE,
	PRIMARY KEY ("name", "user_id")
);


--	triggers

--	--	update timestamp	

	CREATE TRIGGER update_products_timestamp
	BEFORE UPDATE ON "products"
	FOR EACH ROW
	EXECUTE FUNCTION update_updated_at();