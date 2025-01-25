CREATE TABLE IF NOT EXISTS "sessions" (
	"session_id" SERIAL PRIMARY KEY,
	"user_id" INTEGER NOT NULL,
	"created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY ("user_id") REFERENCES users(user_id) ON DELETE CASCADE
);

--	triggers

--	--	update timestamp	

CREATE TRIGGER update_session_timestamp
BEFORE UPDATE ON "sessions"
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();