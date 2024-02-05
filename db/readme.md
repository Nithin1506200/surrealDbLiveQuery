`surreal start --user root --pass root memory`

`surreal import --conn http://localhost:8000 --user root --pass root --ns test --db test db/surreal_deal_v1.surql`

## architecture

- Namespace : test
- db : test

## tables

- users
- groups
- messages

### users - SCHEMAFULL

| column | datatype          | Index                  |
| ------ | ----------------- | ---------------------- |
| id     | primary Id        |                        |
| email  | string::is::email | userEmailIndex::unique |
| name   | String            |                        |
| pass   | String            |                        |

### groups - SCHEMAFULL

| column      | datatype              | Index |
| ----------- | --------------------- | ----- |
| id          | primary               |       |
| name        | string::len() <=30    |       |
| users       | set`<RecordId:users>` |       |
| description | string::len() <=200   |       |
