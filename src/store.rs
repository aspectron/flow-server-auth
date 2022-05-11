use std::collections::HashMap;

pub trait Store: Send + Sync + 'static{
    fn authenticate(&self, username:String, password:String)->(bool, String);
}

pub struct MemoryStore<'a>{
    users:HashMap<&'a str, &'a str>
}

impl<'a> MemoryStore<'a>{
    pub fn new(users:HashMap<&'a str, &'a str>)->Self{
        MemoryStore{
            users
        }
    }
}

impl Store for MemoryStore<'static>{
    fn authenticate(&self, username:String, password:String)->(bool, String){
        if !self.users.contains_key(username.as_str()){
            return (false, "".to_string());
        }

        if let Some(pass) = self.users.get(username.as_str()){
            if pass.eq(&password){
                return (true, username);
            }
        }

        (false, "".to_string())
    }
}

