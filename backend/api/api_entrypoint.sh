#!bash

sleep 5

sqlx migrate run --source "/var/lib/migrations"

cd /app

if [ "$DEBUG" = "true" ]; then
	exec cargo watch -qcx run
else
	exec cargo watch -qcx "run --release"
fi

exit