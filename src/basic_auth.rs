extern crate tide;
pub use async_trait::async_trait;
use crate::Store;
use std::sync::Arc;
use std::fmt::Debug;
use tide::{
    Middleware, Request, Response, Next, Result,
    Server
};

pub struct BasicAuthenticator{
    store:Arc<dyn Store>,
    header_name:String
}

impl BasicAuthenticator{
    pub fn new<S>(store:S)->Self
    where
        S:Store
    {
        Self {
            store:Arc::new(store),
            header_name:"Authorization".to_string()
        }
    }

    pub fn init<State:Clone + Send + Sync + 'static+std::fmt::Debug>(self, app:&mut Server<State>){
        app.with(self);
    }
}

#[async_trait]
impl<'a, State> Middleware<State> for BasicAuthenticator
where
    State: Clone + Send + Sync + Debug + 'static,
{
    async fn handle(&self, mut req: Request<State>, next: Next<'_, State>) -> Result {
        println!("headers {:#?}", req);
        let headers = req.header(self.header_name.as_str());
        if headers.is_none(){
            let mut res = Response::new(401);
            res.append_header("WWW-Authenticate", "Basic");
            return Ok(res);
        }



        println!("headers: {:?}", headers);
        println!("state: {:?}", req.state());
        Ok(Response::from(""))
    }
}
