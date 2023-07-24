export CONFIG_SOURCE=nsuns-server/config/settings.toml
export DATABASE_MIGRATIONS=nsuns-server/db/migrations
export STATIC_FILES_DIR=dist/assets

cargo run nsuns-server "$@"
