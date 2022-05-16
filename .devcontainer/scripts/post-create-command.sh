#!/bin/sh

-e

if [ ! -f ./.devcontainer/.env ]
then
  export $(cat ./.devcontainer/.env | xargs)
fi

cargo install cargo-edit
cargo install sqlx-cli --no-default-features --features native-tls,postgres

export DATABASE_URL="postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_HOSTNAME:$POSTGRES_PORT/$POSTGRES_DB"