-- Add up migration script here
CREATE TABLE IF NOT EXISTS "order_details" (
	"id" SERIAL PRIMARY KEY,
	"delivery_address" VARCHAR(255) NOT NULL,
	"created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


--	triggers

--	--	update timestamp	

	CREATE TRIGGER update_order_details_timestamp
	BEFORE UPDATE ON "order_details"
	FOR EACH ROW
	EXECUTE FUNCTION update_updated_at();