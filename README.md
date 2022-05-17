# DDD in Rust

## Migration

First, run commands below to migrate.

```sh

sqlx migrate add user --source ./sql

sqlx migrate add extensions --source ./sql

```

Add migration script like so:

`xxxxxxx_user.sql`
```sql

CREATE TABLE IF NOT EXISTS public.user (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL
);

```

`xxxxxxx_extensions.sql`
```sql

CREATE EXTENSION IF NOT EXISTS pgcrypto;

```

And then, run the command below

```sh

sqlx migrate run --source ./sql

```

## References

* https://zenn.dev/htlsne/articles/rust-sqlx-test