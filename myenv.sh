POSTGRES_PASSWORD="$(gopass show -o Playground/postgre_sample)"
export POSTGRES_PASSWORD
export POSTGRES_VERSION='latest'
export POSTGRES_USER='postgres'
export POSTGRES_DB='postgres'
export CONN_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/${POSTGRES_DB}"
