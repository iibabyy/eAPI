#!/usr/bin/bash

set -e

# URL example : postgres://user:pass@host:port/db
DB_HOST=${POSTGRES_HOST:-localhost}
DB_PORT=${POSTGRES_PORT:-5432}

echo waiting for "'$DB_HOST:$DB_PORT'"

until pg_isready -h "$DB_HOST" -p "$DB_PORT" >/dev/null 2>&1; do
  echo "Database not ready — new attempt in 2s..."
  sleep 2
done

echo "Database ready !"

export DATABASE_URL="postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$DB_HOST:$DB_PORT/${POSTGRES_DB:-eapi}"
exec "$@"