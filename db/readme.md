`surreal start --user root --pass root memory`

`surreal import --conn http://localhost:8000 --user root --pass root --ns test --db test db/surreal_deal_v1.surql`

## architecture

- Namespace : test
- db : test

## tables

- users
- order
- transaction

### merchant - schemafull

| column | datatype                            | Index                   |
| ------ | ----------------------------------- | ----------------------- |
| email  | string::is::email                   | adminEmailIndex::unique |
| id     | CustomId:`merchant:<merchant_name>` |                         |
| name   | string                              | adminNameIndex::unique  |

### users

| column      | datatype            | Index                  |
| ----------- | ------------------- | ---------------------- |
| email       | string::is::email   | userEmailIndex::unique |
| merchant_id | RecordID:`merchant` | userAdminIndex         |
| name        | String              |                        |
| pass        | String              |                        |

### order_reference

| column     | datatype    | Comments |
| ---------- | ----------- | -------- |
| amount     | float       |          |
| created_at | datestring  |          |
| status     | ENUM:STATUS |          |
| admin_id   |             |          |
| updated_at | datestring  |          |

### transactions

| column     | datatype                   | comments |
| ---------- | -------------------------- | -------- |
| amount     | float                      |          |
| created_at | datestring                 |          |
| order_id   | RecordID:`order_reference` |          |
| status     | Enum:STATUS                |          |
| updated_at | datestring                 |          |

\*\* node signup should be not allowed from external
