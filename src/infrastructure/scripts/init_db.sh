#!/usr/bin/env bash
set -x
set -eo pipefail

# Database configuration
DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=test}"
DB_NAME="${POSTGRES_DB:=testdb}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"

# Allow to skip Podman if a Podman-managed Postgres container is already running
if [[ -z "${SKIP_PODMAN}" ]]; then
    podman run --name postgres-container \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d docker.io/library/postgres:latest \
        postgres -N 1000
fi

# Wait for PostgreSQL to become available
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and running on ${DB_HOST}:${DB_PORT} - database ${DB_NAME} is ready to use"

# Export DATABASE_URL for the backend to use
DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
export DATABASE_URL
>&2 echo "DATABASE_URL=${DATABASE_URL}"
