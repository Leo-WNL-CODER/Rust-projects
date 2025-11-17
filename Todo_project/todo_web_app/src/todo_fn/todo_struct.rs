use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};


#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct Task{
    pub id:u32,
    pub description:String,
    pub completed:bool
}

#[derive(Clone,Debug)]
pub struct Todos{
   pub todo:Arc<Mutex<Vec<Task>>>,
   pub next_id:Arc<Mutex<u32>>
}


#[derive(Serialize ,Deserialize ,Debug,Clone)]
pub struct PayloadRequest{
    pub description:String
}

