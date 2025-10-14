--		import uuid
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

--		update upload timestam
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
	NEW."updated_at" = NOW();
	RETURN NEW;
END;
$$ LANGUAGE plpgsql;