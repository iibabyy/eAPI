-- Add up migration script here
CREATE TABLE IF NOT EXISTS "users" (
	"user_id" SERIAL PRIMARY KEY,
	"email" VARCHAR(255) NOT NULL CHECK("email" <> '') UNIQUE,
	"password" VARCHAR(255) NOT NULL CHECK("password" <> ''),
	"username" VARCHAR(255) NOT NULL CHECK("username" <> '') UNIQUE,
	"sold" INTEGER NOT NULL DEFAULT 0,
	"day_requests" INTEGER NOT NULL DEFAULT 0,
	"minute_requests" INTEGER NOT NULL DEFAULT 0,
	"created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);



--	triggers

--	--	update timestamp	

CREATE TRIGGER update_users_timestamp
BEFORE UPDATE ON "users"
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();