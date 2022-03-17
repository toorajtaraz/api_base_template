use crate::schema::tokens;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "tokens"]
pub struct NewToken {
    pub user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TokenData {
    pub user_id: i32,
}
