-- Add up migration script here
CREATE TABLE IF NOT EXISTS "users" (
	"id" SERIAL PRIMARY KEY,
	"email" VARCHAR(255) NOT NULL UNIQUE,
	"password" VARCHAR(255) NOT NULL,
	"username" VARCHAR(255) NOT NULL,
	"sold" INTEGER NOT NULL
);