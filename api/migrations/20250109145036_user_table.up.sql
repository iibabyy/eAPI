-- Add up migration script here
CREATE TABLE IF NOT EXISTS "users" (
	"user_id" SERIAL PRIMARY KEY,
	"email" VARCHAR(255) NOT NULL UNIQUE,
	"password" VARCHAR(255) NOT NULL,
	"username" VARCHAR(255) NOT NULL,
	"sold" INTEGER NOT NULL DEFAULT 0,
	"created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);



--	triggers

--	--	update timestamp	

CREATE TRIGGER update_users_timestamp
BEFORE UPDATE ON "users"
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();