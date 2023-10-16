OPTION IMPORT;
DEFINE TABLE orders SCHEMALESS PERMISSIONS 
    FOR select WHERE merchant = $auth.id; 
DEFINE FIELD merchant ON orders TYPE string;
DEFINE INDEX merchantIndex On TABLE orders COLUMNS merchant;
DEFINE FIELD status ON orders TYPE string;
DEFINE INDEX statusIndex On TABLE orders COLUMNS string;
DEFINE FIELD amount ON orders TYPE number;
DEFINE FIELD created_on ON TABLE orders type datetime;
