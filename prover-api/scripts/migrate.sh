#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd -- "$SCRIPT_DIR/.." && pwd)"

if ! command -v psql >/dev/null 2>&1; then
    printf 'error: psql is required but was not found in PATH\n' >&2
    exit 1
fi

if [[ -z "${DATABASE_URL:-}" && -f "$PROJECT_ROOT/.env" ]]; then
    DATABASE_URL="$(awk -F= '$1 == "DATABASE_URL" {
        print substr($0, index($0, "=") + 1)
        exit
    }' "$PROJECT_ROOT/.env")"
fi

if [[ -z "${DATABASE_URL:-}" ]]; then
    printf 'error: DATABASE_URL is not set and was not found in .env\n' >&2
    exit 1
fi

psql "$DATABASE_URL" --set ON_ERROR_STOP=1 <<'SQL'
BEGIN;

CREATE TABLE IF NOT EXISTS schema_migrations (
    version TEXT PRIMARY KEY,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS github_users (
    github_id BIGINT PRIMARY KEY,
    login TEXT NOT NULL,
    email TEXT,
    avatar_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS sessions (
    token_hash TEXT PRIMARY KEY,
    github_id BIGINT NOT NULL REFERENCES github_users(github_id),
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at TIMESTAMPTZ
);

INSERT INTO schema_migrations (version)
VALUES ('001_create_github_users')
ON CONFLICT (version) DO NOTHING;

INSERT INTO schema_migrations (version)
VALUES ('002_create_sessions')
ON CONFLICT (version) DO NOTHING;

COMMIT;
SQL

printf 'database migration completed successfully\n'
