# Surreal db with live query and authentication

- make single namespace , memory layer live query

- db : [db](/db/readme.md)

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
