OPTION IMPORT;

-- merchants
DEFINE TABLE merchants SCHEMAFULL PERMISSIONS NONE;
DEFINE FIELD email ON merchants TYPE string ASSERT string::is::email($value);
DEFINE FIELD name ON merchants TYPE string ASSERT $value!=NONE;
-- indexes
DEFINE INDEX adminEmailIndex ON merchants COLUMNS email UNIQUE;


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
UPDATE merchant:company_x content {
    email:"company_x@mail.com",
    name:"Company X limited"
};
UPDATE merchant:company_y content {
    email:"company_y@mail.com",
    name:"Company Y limited"
};
CREATE users SET 
    email="user1@company_x.com",
    name="User 1",
    pass=crypto::argon2::generate("user1@123"),
    merchant_id=merchant:company_x
;
CREATE users SET 
    email="user2@company_x.com",
    name="User 2",
    pass=crypto::argon2::generate("user2@123"),
    merchant_id=merchant:company_y
;
COMMIT TRANSACTION;