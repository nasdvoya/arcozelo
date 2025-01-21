## Intro

Please read "Zero to Production in Rust" by Luca Palmieri.

## Database

This project uses **PostgreSQL** as database with **sqlx** crate to interact with **PostgresSQL**.

```bash

# Tooling (you can also use flake.nix)
cargo install --version="~0.7" sqlx-cli --no-default-features \
--features rustls,postgres

# Create a migration with
sqlx migrate add create_some_table

# Run
./scripts/init_db.sh
# Or just migrate
SKIP_DOCKER=true ./scripts/init_db.sh

```
