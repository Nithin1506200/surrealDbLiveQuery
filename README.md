# Surreal db with live query and authentication

- make single namespace , memory layer live query

- db : [db](/db/readme.md)
- install surreal db
- install rust

Step 1 : setup db

- start db : `surreal start --user root --pass root memory`
- migrate db : `surreal import --conn http://localhost:8000 --user root --pass root --ns test --db test db/migrationV1.surql`

Step 2 : start backend

```sh
cd server
cargo run
```

Step 3 : start FE

## Tech stacks

- Frontend : React , Vite , Typescript
- Backend : Rust , actix-web
- Database : SurrealDb

## enums

- status : `NEW` | `SUCCESS` | `FAILED`

## Default users

| user name | password | email                | role   | access                |
| --------- | -------- | -------------------- | ------ | --------------------- |
| root      | root     | <root@surreal.com>   | admin  | \*                    |
| viewer    | viewer   | <viewer@surreal.com> | viewer | ns:"test" db : "test" |
|           |
