use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User{
    pub uid:String,
    pub username:String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BasicUser{
    pub uid:String
}