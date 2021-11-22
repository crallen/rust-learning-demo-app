#!/usr/bin/env bash

set -euo pipefail

function print_help() {
  echo "Utility to manage the database and migrations"
  echo
  echo "Usage: ./migrate.sh [options] <command>"
  echo
  echo "Commands:"
  echo "  init          Creates the database and runs all pending migrations"
  echo "  new <name>    Creates new up/down migration files with the name specified"
  echo "  up            Runs all pending migrations"
  echo "  down          Reverts the latest migration"
  echo "  reset         Destroys the database, re-creates it, and runs all pending migrations"
  echo
  echo "Options:"
  echo "  -h --help     Prints this help text"
}

function error() {
    echo -e "\033[0;31m$1\033[0m" >&2
}

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

    -h|--help)
      print_help
      exit 0
      ;;

    *)
      error "Unrecognized argument: $1"
      exit 1
      ;;
  esac
  shift
done

export DATABASE_URL

eval "sqlx $SQLX_COMMAND"