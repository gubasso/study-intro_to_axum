# ripissue draft

- [x] docker secrets
- [ ] organize myenv.sh

export PGPASSWORD="$(cat /run/secrets/postgres_password)"
export PGPASSWORD="xuranha"
psql -U "$POSTGRES_USER" -d "$POSTGRES_DB"

docker cp intro_to_axum-db-1:/var/lib/postgresql/data/pg_hba.conf ./pg_hba.conf
docker cp  ./pg_hba.conf intro_to_axum-db-1:/var/lib/postgresql/data/pg_hba.conf
