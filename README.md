# Groshekeli spendings service

This service is in charge of handling spendings.

### Environment variables

- GROSH_DB_URL - database url. For example `postgres://postgres:grosh@127.0.0.1/groshekeli`
- GROSH_SPENDINGS_SCHEMA - database schema for given service. 
- GROSH_SPENDINGS_BIND_ADDRESS - address to listen on. For example "0.0.0.0:8080"

This variables can be found in `src/infrastructure/config`.

### Running

Start Postgres, create `'spendings'` schema and `make run`.
