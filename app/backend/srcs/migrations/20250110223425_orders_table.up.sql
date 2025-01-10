-- Add up migration script here
CREATE TABLE IF NOT EXISTS "orders" (
	"id" SERIAL PRIMARY KEY,
	"user_id" INTEGER NOT NULL,
	"product_id" INTEGER NOT NULL
);