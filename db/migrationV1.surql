OPTION IMPORT;

-- merchants
DEFINE TABLE account SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD email ON account TYPE string ASSERT string::is::email($value);
DEFINE FIELD name ON account TYPE string ASSERT $value!=NONE;
-- indexes
DEFINE INDEX adminEmailIndex ON account COLUMNS email UNIQUE;


-- users ---------
DEFINE TABLE users SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD email ON users TYPE string ASSERT string::is::email($value);
DEFINE FIELD pass ON users TYPE string ASSERT $value!=NONE;
DEFINE FIELD name ON users TYPE string ASSERT $value!=NONE;
-- Record link for merchants table
DEFINE FIELD merchant_id ON users TYPE string ASSERT $value!=NONE;
-- indexes
DEFINE INDEX userEmailIndex ON users COLUMNS email UNIQUE;
DEFINE INDEX userMerchantIndex ON users COLUMNS merchant_id;

BEGIN TRANSACTION;
UPDATE account:account1 content {
    email:"account1@mail.com",
    name:"Account 1 limited"
};
UPDATE account:account2 content {
    email:"account2@mail.com",
    name:"Account 2 limited"
};
CREATE users SET 
    email="user1@account1.com",
    name="User 1",
    pass=crypto::argon2::generate("user1@123"),
    merchant_id=account:account1
;
CREATE users SET 
    email="user2@account2.com",
    name="User 2",
    pass=crypto::argon2::generate("user2@123"),
    merchant_id=account:account2
;
COMMIT TRANSACTION;