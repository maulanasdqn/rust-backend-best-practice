#!/bin/sh
set -e

echo "Starting Financial Tracker API..."
echo "Waiting for database to be ready..."
until pg_isready -h db -p 5432 -U postgres; do
  echo "Database is unavailable - sleeping"
  sleep 1
done

echo "Database is ready!"
echo "Running database migrations..."
if [ -z "${SKIP_MIGRATIONS}" ]; then
  /fta-migration || {
    echo "Migration failed, attempting with sqlx..."
    /sqlx migrate run --database-url "${DATABASE_URL}" --source /migrations || {
      echo "ERROR: Database migration failed"
      exit 1
    }
  }
  echo "Migrations completed successfully!"
else
  echo "Skipping migrations (SKIP_MIGRATIONS is set)"
fi

echo "Starting API server..."
exec /fta-server
