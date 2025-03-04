### For installation
cargo install diesel_cli --no-default-features --features postgres

### To run postgres docker
docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 -d postgres

### to setup the .env
echo DATABASE_URL=postgres://username:password@localhost/db_name > .env

### setup diesel
diesel setup
<!-- diesel migration generate create_ask_records (already present then no need) -->
diesel migration run
diesel migration list
