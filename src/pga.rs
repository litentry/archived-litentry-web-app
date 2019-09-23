use seed::prelude::*;
use seed::{fetch, Method, Request};
use serde::{Serialize, Deserialize};
use serde_json::json;
use futures::Future;


pub struct Model {
}

impl Default for Model {
    fn default() -> Self {
        Self {
        }
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub enum Msg {
}


pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {

    }
}


pub fn view(model: &Model) -> Node<Msg> {
    div![id!("pga"),
         div![class!["account"],
              span!["Your Account: "],
              span!["0x5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
         ],
         div![class!["destination"],
              "Destination Account: ",
              span![""]
         ],
         div![class!["issued_identity"],
              "Issued Identity: ",
              span![""]
         ],
         div![class!["data"],
              "Success or Not!",
              span![""]
         ],
         div![
             button!["Generate!"]
         ]
    ]
}
