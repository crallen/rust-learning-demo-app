#!/usr/bin/env bash

set -euo pipefail

if [[ -f ".env" ]]; then
  source ".env"
fi

DB_HOST="${DB_HOST:-localhost}"
DB_PORT="${DB_PORT:-5432}"
DB_USER="${DB_USER:-identity}"
DB_PASSWORD="${DB_PASSWORD:-}"
DB_NAME="${DB_NAME:-identity}"
DATABASE_URL="postgres://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME"
SQLX_COMMAND=

while [[ $# -gt 0 ]]
do
  case $1 in
    init)
      SQLX_COMMAND="database setup"
      ;;

    new)
      SQLX_COMMAND="migrate add -r $2"
      shift
      ;;

    up)
      SQLX_COMMAND="migrate run"
      ;;

    down)
      SQLX_COMMAND="migrate revert"
      ;;

    reset)
      SQLX_COMMAND="database reset"
      ;;
  esac
  shift
done

export DATABASE_URL

eval "sqlx $SQLX_COMMAND"