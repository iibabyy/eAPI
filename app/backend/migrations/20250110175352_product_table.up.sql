-- Add up migration script here
CREATE TABLE IF NOT EXISTS "products" (
	"id" SERIAL PRIMARY KEY,
	"name" VARCHAR(255) NOT NULL UNIQUE,
	"price" INTEGER NOT NULL,
	"owner_id" INTEGER NOT NULL
);