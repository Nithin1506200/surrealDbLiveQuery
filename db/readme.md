`surreal start --user root --pass root memory`
`surreal import --conn http://localhost:8000 --user root --pass root --ns test --db test db/surreal_deal_v1.surql`

## architecture

- Namespace : liveQuery
- db : liveQuery

## tables

- merchants
- users
- order
- transaction

### merchants - schemafull

| column | datatype                            | comments |
| ------ | ----------------------------------- | -------- |
| id     | CustomId:`merchant:<merchant_name>` | -------- |
| email  | string::is::email                   | email    |

### users

| column      | datatype             | Comments |
| ----------- | -------------------- | -------- |
| merchant_id | RecordID:`merchants` |          |
| email       | string::is::email    | unique   |
| pass        |                      |          |

### order_reference

| column     | datatype    | Comments |
| ---------- | ----------- | -------- |
| status     | ENUM:STATUS |          |
| amount     | float       |          |
| created_at | datestring  |          |
| updated_at | datestring  |          |

### transactions

| column     | datatype                   | comments |
| ---------- | -------------------------- | -------- |
| status     | Enum:STATUS                |          |
| order_id   | RecordID:`order_reference` |          |
| amount     | float                      |          |
| created_at | datestring                 |          |
| updated_at | datestring                 |          |

\*\* node signup should be not allowed from external
